use crate::build::build_site;
use crate::config::Config;
use crate::error::{MiniZensicalError, Result};
use percent_encoding::percent_decode_str;
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, UNIX_EPOCH};
use walkdir::WalkDir;

pub const DEFAULT_PREVIEW_ADDR: &str = "127.0.0.1:3000";

pub fn serve_site(config: &Config, addr: &str) -> Result<()> {
    build_site(config)?;

    let site_dir = Arc::new(RwLock::new(config.site_dir()));
    spawn_watch_thread(config.clone(), site_dir.clone());

    let listener =
        TcpListener::bind(addr).map_err(|error| MiniZensicalError::io("bind", addr, error))?;

    println!(
        "Serving {} at http://{addr}",
        site_dir
            .read()
            .expect("site dir lock should not be poisoned")
            .display()
    );
    println!("Press Ctrl+C to stop.");
    println!("Watching docs and configuration for changes...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let current_site_dir = site_dir
                    .read()
                    .expect("site dir lock should not be poisoned")
                    .clone();
                if let Err(error) = handle_connection(stream, &current_site_dir) {
                    eprintln!("Request error: {error}");
                }
            }
            Err(error) => {
                eprintln!("Accept error: {error}");
            }
        }
    }

    Ok(())
}

fn spawn_watch_thread(initial_config: Config, served_site_dir: Arc<RwLock<PathBuf>>) {
    thread::spawn(move || {
        let config_path = initial_config.path.clone();
        let mut current_config = initial_config;
        let mut snapshot = snapshot_sources(&current_config);

        loop {
            thread::sleep(Duration::from_millis(800));
            let latest_snapshot = snapshot_sources(&current_config);
            if latest_snapshot == snapshot {
                continue;
            }
            snapshot = latest_snapshot;

            let next_config = match Config::load(&config_path) {
                Ok(config) => config,
                Err(error) => {
                    eprintln!("Rebuild skipped: {error}");
                    continue;
                }
            };

            println!("Change detected. Rebuilding...");
            let previous_site_dir = served_site_dir
                .read()
                .expect("site dir lock should not be poisoned")
                .clone();

            match build_site(&next_config) {
                Ok(()) => {
                    let next_site_dir = next_config.site_dir();
                    if previous_site_dir != next_site_dir && previous_site_dir.exists() {
                        let _ = fs::remove_dir_all(&previous_site_dir);
                    }

                    current_config = next_config;
                    snapshot = snapshot_sources(&current_config);
                    *served_site_dir
                        .write()
                        .expect("site dir lock should not be poisoned") = current_config.site_dir();
                    println!("Rebuild finished.");
                }
                Err(error) => {
                    current_config = next_config;
                    snapshot = snapshot_sources(&current_config);
                    eprintln!("Rebuild failed: {error}");
                    eprintln!("Serving the last successful build.");
                }
            }
        }
    });
}

fn handle_connection(mut stream: TcpStream, site_dir: &Path) -> io::Result<()> {
    let request_line = read_request_line(&stream)?;
    if request_line.is_empty() {
        return Ok(());
    }

    let Some((method, target)) = parse_request_line(&request_line) else {
        return write_error_response(&mut stream, 400, "Bad Request", "Malformed request.");
    };

    if method != "GET" && method != "HEAD" {
        return write_error_response(
            &mut stream,
            405,
            "Method Not Allowed",
            "Only GET and HEAD are supported.",
        );
    }

    let Some(file_path) = resolve_request_path(site_dir, target) else {
        return write_error_response(&mut stream, 404, "Not Found", "File not found.");
    };

    match fs::read(&file_path) {
        Ok(body) => {
            let body = if method == "HEAD" { Vec::new() } else { body };
            write_response(&mut stream, 200, "OK", content_type_for(&file_path), &body)
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            write_error_response(&mut stream, 404, "Not Found", "File not found.")
        }
        Err(error) => Err(error),
    }
}

fn read_request_line(stream: &TcpStream) -> io::Result<String> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    let mut header_line = String::new();
    loop {
        header_line.clear();
        let read = reader.read_line(&mut header_line)?;
        if read == 0 || header_line == "\r\n" {
            break;
        }
    }

    Ok(request_line)
}

fn parse_request_line(request_line: &str) -> Option<(&str, &str)> {
    let mut parts = request_line.split_whitespace();
    let method = parts.next()?;
    let target = parts.next()?;
    let _version = parts.next()?;
    Some((method, target))
}

fn write_response(
    stream: &mut TcpStream,
    status_code: u16,
    status_text: &str,
    content_type: &str,
    body: &[u8],
) -> io::Result<()> {
    write!(
        stream,
        "HTTP/1.1 {status_code} {status_text}\r\n\
         Content-Length: {}\r\n\
         Content-Type: {content_type}\r\n\
         Connection: close\r\n\r\n",
        body.len()
    )?;
    if !body.is_empty() {
        stream.write_all(body)?;
    }
    stream.flush()
}

fn write_error_response(
    stream: &mut TcpStream,
    status_code: u16,
    status_text: &str,
    message: &str,
) -> io::Result<()> {
    let body = format!(
        "<!doctype html><html><body><h1>{status_code} {status_text}</h1><p>{message}</p></body></html>"
    );
    write_response(
        stream,
        status_code,
        status_text,
        "text/html; charset=utf-8",
        body.as_bytes(),
    )
}

pub(crate) fn resolve_request_path(site_dir: &Path, target: &str) -> Option<PathBuf> {
    let raw_path = target.split('?').next().unwrap_or("/");
    let decoded = percent_decode_str(raw_path).decode_utf8_lossy();
    let trimmed = decoded.trim_start_matches('/');
    let request_path = if trimmed.is_empty() {
        PathBuf::from("index.html")
    } else {
        PathBuf::from(trimmed)
    };

    let safe_path = sanitize_relative_path(&request_path)?;

    let direct = site_dir.join(&safe_path);
    if direct.is_file() {
        return Some(direct);
    }

    let dir_index = site_dir.join(&safe_path).join("index.html");
    if dir_index.is_file() {
        return Some(dir_index);
    }

    if safe_path.extension().is_none() {
        let mut html_path = safe_path.clone();
        html_path.set_extension("html");
        let html_file = site_dir.join(html_path);
        if html_file.is_file() {
            return Some(html_file);
        }
    }

    None
}

fn sanitize_relative_path(path: &Path) -> Option<PathBuf> {
    let mut sanitized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => sanitized.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    Some(sanitized)
}

fn content_type_for(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("webp") => "image/webp",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

fn snapshot_sources(config: &Config) -> BTreeMap<PathBuf, u128> {
    let mut snapshot = BTreeMap::new();

    add_file_timestamp(&mut snapshot, &config.path);

    let docs_dir = config.docs_dir();
    if docs_dir.exists() {
        for entry in WalkDir::new(&docs_dir).sort_by_file_name() {
            let Ok(entry) = entry else {
                continue;
            };
            if entry.file_type().is_file() {
                add_file_timestamp(&mut snapshot, entry.path());
            }
        }
    }

    snapshot
}

fn add_file_timestamp(snapshot: &mut BTreeMap<PathBuf, u128>, path: &Path) {
    if let Ok(metadata) = fs::metadata(path) {
        if let Ok(modified) = metadata.modified() {
            let timestamp = modified
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_millis())
                .unwrap_or(0);
            snapshot.insert(path.to_path_buf(), timestamp);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{resolve_request_path, snapshot_sources};
    use crate::config::Config;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn resolves_root_directory_and_html_requests() {
        let temp_dir = TempDir::new().unwrap();
        let site_dir = temp_dir.path();

        fs::create_dir_all(site_dir.join("guide/setup")).unwrap();
        fs::write(site_dir.join("index.html"), "home").unwrap();
        fs::write(site_dir.join("guide/setup/index.html"), "setup").unwrap();
        fs::write(site_dir.join("page.html"), "page").unwrap();

        assert_eq!(
            resolve_request_path(site_dir, "/").unwrap(),
            site_dir.join("index.html")
        );
        assert_eq!(
            resolve_request_path(site_dir, "/guide/setup/").unwrap(),
            site_dir.join("guide/setup/index.html")
        );
        assert_eq!(
            resolve_request_path(site_dir, "/page").unwrap(),
            site_dir.join("page.html")
        );
    }

    #[test]
    fn resolves_percent_encoded_unicode_assets() {
        let temp_dir = TempDir::new().unwrap();
        let site_dir = temp_dir.path();

        fs::create_dir_all(site_dir.join("assets")).unwrap();
        let file = site_dir.join("assets/交大校徽-蓝色.png");
        fs::write(&file, "png").unwrap();

        let encoded = "/assets/%E4%BA%A4%E5%A4%A7%E6%A0%A1%E5%BE%BD-%E8%93%9D%E8%89%B2.png";
        assert_eq!(resolve_request_path(site_dir, encoded).unwrap(), file);
    }

    #[test]
    fn rejects_parent_directory_requests() {
        let temp_dir = TempDir::new().unwrap();
        assert!(resolve_request_path(temp_dir.path(), "/../../etc/passwd").is_none());
    }

    #[test]
    fn snapshots_config_and_docs_files() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("zensical.toml"),
            "[project]\nsite_name = \"Docs\"\n",
        )
        .unwrap();
        fs::create_dir_all(temp_dir.path().join("docs/guide")).unwrap();
        fs::write(temp_dir.path().join("docs/index.md"), "# Home\n").unwrap();
        fs::write(temp_dir.path().join("docs/guide/setup.md"), "# Setup\n").unwrap();

        let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
        let snapshot = snapshot_sources(&config);

        assert!(snapshot.contains_key(&temp_dir.path().join("zensical.toml")));
        assert!(snapshot.contains_key(&temp_dir.path().join("docs/index.md")));
        assert!(snapshot.contains_key(&temp_dir.path().join("docs/guide/setup.md")));
    }
}
