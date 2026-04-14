use crate::error::{MiniZensicalError, Result};
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Config {
    pub path: PathBuf,
    pub root_dir: PathBuf,
    pub project: ProjectConfig,
}

#[derive(Clone, Debug, Deserialize)]
struct RawConfig {
    project: ProjectConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProjectConfig {
    pub site_name: String,
    #[serde(default = "default_docs_dir")]
    pub docs_dir: String,
    #[serde(default = "default_site_dir")]
    pub site_dir: String,
    #[serde(default = "default_true")]
    pub use_directory_urls: bool,
    pub site_url: Option<String>,
    #[serde(default)]
    pub nav: Vec<NavItemConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NavItemConfig {
    pub title: String,
    pub path: Option<String>,
    #[serde(default)]
    pub children: Vec<NavItemConfig>,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = absolute_path(path.as_ref())?;
        let data = fs::read_to_string(&path)
            .map_err(|error| MiniZensicalError::io("read", &path, error))?;
        let raw: RawConfig =
            toml::from_str(&data).map_err(|error| MiniZensicalError::ParseConfig {
                path: path.clone(),
                source: error,
            })?;

        let root_dir = path.parent().map(Path::to_path_buf).ok_or_else(|| {
            MiniZensicalError::InvalidConfig(String::from(
                "configuration file must live inside a directory",
            ))
        })?;

        let config = Self {
            path,
            root_dir,
            project: raw.project,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn docs_dir(&self) -> PathBuf {
        self.root_dir.join(&self.project.docs_dir)
    }

    pub fn site_dir(&self) -> PathBuf {
        self.root_dir.join(&self.project.site_dir)
    }

    pub fn docs_path_for(&self, relative_path: impl AsRef<Path>) -> PathBuf {
        self.docs_dir().join(relative_path)
    }

    pub fn site_path_for(&self, relative_path: impl AsRef<Path>) -> PathBuf {
        self.site_dir().join(relative_path)
    }

    fn validate(&self) -> Result<()> {
        if self.project.site_name.trim().is_empty() {
            return Err(MiniZensicalError::InvalidConfig(String::from(
                "project.site_name cannot be empty",
            )));
        }

        validate_relative_dir("project.docs_dir", &self.project.docs_dir)?;
        validate_relative_dir("project.site_dir", &self.project.site_dir)?;

        if self.project.docs_dir == self.project.site_dir {
            return Err(MiniZensicalError::InvalidConfig(String::from(
                "project.docs_dir and project.site_dir must be different",
            )));
        }

        for item in &self.project.nav {
            validate_nav_item(item)?;
        }

        Ok(())
    }
}

fn absolute_path(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }

    let current_dir = env::current_dir()
        .map_err(|error| MiniZensicalError::io("read current directory", ".", error))?;
    Ok(current_dir.join(path))
}

fn validate_relative_dir(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(MiniZensicalError::InvalidConfig(format!(
            "{label} cannot be empty"
        )));
    }

    let path = Path::new(value);
    if path.is_absolute() {
        return Err(MiniZensicalError::InvalidConfig(format!(
            "{label} must be relative to zensical.toml"
        )));
    }

    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(MiniZensicalError::InvalidConfig(format!(
            "{label} cannot contain parent-directory segments"
        )));
    }

    Ok(())
}

fn validate_nav_item(item: &NavItemConfig) -> Result<()> {
    if item.title.trim().is_empty() {
        return Err(MiniZensicalError::InvalidConfig(String::from(
            "every nav item must have a non-empty title",
        )));
    }

    match (item.path.as_ref(), item.children.is_empty()) {
        (Some(_), true) => Ok(()),
        (None, false) => {
            for child in &item.children {
                validate_nav_item(child)?;
            }
            Ok(())
        }
        (Some(_), false) => Err(MiniZensicalError::InvalidConfig(format!(
            "nav item '{}' cannot define both path and children",
            item.title
        ))),
        (None, true) => Err(MiniZensicalError::InvalidConfig(format!(
            "nav item '{}' must define either path or children",
            item.title
        ))),
    }
}

fn default_docs_dir() -> String {
    String::from("docs")
}

fn default_site_dir() -> String {
    String::from("site")
}

fn default_true() -> bool {
    true
}
