# Rustring

Rustring 是一个用 Rust 编写的静态文档站点生成器。它读取 Markdown 文档和一个 `zensical.toml` 配置文件，生成可以直接部署的静态网站，并提供搜索、自动导航、归档、主题切换、代码工具、数学公式和知识图谱等功能。

你不需要把自己的文档放进 Rustring 的源码仓库。安装命令行工具后，可以在任意目录创建并管理独立的文档项目。

## 功能概览

- 将 Markdown 文件构建为静态 HTML 网站
- 自动导航或通过配置显式定义导航
- 全文搜索，并跳转和高亮具体命中段落
- 按日期和标签自动生成归档页面
- 根据文档链接和标签生成交互式知识图谱
- 支持 YAML Front Matter
- 支持代码高亮和一键复制
- 支持行内公式和块级公式
- 支持图片、字体和其他静态资源
- 提供 `Light`、`Dark`、`System`、`Sepia`、`Ocean`、`Forest` 主题
- 本地预览时自动重建并刷新页面

## 安装

Rustring 目前通过 Cargo 从 GitHub 安装。Cargo 是 Rust 的包管理和构建工具，因此需要先安装 Rust。

### 1. 安装 Rust

- macOS：按照 [Rust 官方安装页面](https://www.rust-lang.org/tools/install) 的说明使用 `rustup` 安装。
- Windows：从 [Rust 官方安装页面](https://www.rust-lang.org/tools/install) 下载并运行 `rustup-init.exe`。安装程序可能会提示安装 Visual Studio C++ Build Tools。

安装完成后，重新打开终端并检查：

```bash
rustc --version
cargo --version
```

### 2. 安装 Rustring

在 macOS Terminal、Windows PowerShell 或 Windows Terminal 中运行：

```bash
cargo install --git https://github.com/TZMydl2006/Rustring.git --bin rustring
```

确认安装成功：

```bash
rustring --version
rustring --help
```

Cargo 默认把可执行文件安装到 `$HOME/.cargo/bin`。使用 `rustup` 安装 Rust 时，该目录通常会自动加入 `PATH`。如果终端提示找不到 `rustring`，请重新打开终端并检查 Cargo 的 `bin` 目录是否在 `PATH` 中。

### 更新与卸载

更新到仓库中的最新版本：

```bash
cargo install --git https://github.com/TZMydl2006/Rustring.git --bin rustring --force
```

卸载：

```bash
cargo uninstall minizensical
```

`minizensical` 是当前 Rust package 名称，安装后的命令名称是 `rustring`。

## 快速开始

### 1. 创建项目目录

```bash
mkdir my-docs
cd my-docs
```

PowerShell 中也可以使用相同命令；如果目录已经存在，直接进入该目录即可。

### 2. 初始化文档项目

```bash
rustring init
```

该命令会创建：

```text
my-docs/
├── zensical.toml
└── docs/
    └── index.md
```

已有的 `zensical.toml` 和 `docs/index.md` 不会被覆盖。

### 3. 本地预览

```bash
rustring serve
```

在浏览器中打开：

```text
http://127.0.0.1:3000
```

修改 Markdown、配置或静态资源后，Rustring 会自动重建站点；成功重建后，已打开的页面会自动刷新。

使用其他地址或端口：

```bash
rustring serve --addr 127.0.0.1:4000
```

按 `Ctrl+C` 停止预览服务器。

### 4. 构建静态网站

```bash
rustring build
```

默认构建结果位于 `site/`：

```text
my-docs/
├── docs/
├── site/
│   ├── index.html
│   ├── search.json
│   ├── graph.json
│   ├── archive/
│   ├── knowledge-graph/
│   └── assets/
└── zensical.toml
```

`site/` 是生成目录。每次构建都可能重建其中的内容，因此不要直接编辑里面的文件；应修改 `docs/` 或 `zensical.toml`。

## 在任意位置使用

Rustring 默认读取当前目录中的 `zensical.toml`。文档项目可以位于任何目录，不需要位于 Rustring 源码中。

如果当前终端不在项目目录，可以指定配置文件：

```bash
rustring build --config /path/to/project/zensical.toml
rustring serve --config /path/to/project/zensical.toml
```

Windows PowerShell 示例：

```powershell
rustring build --config C:\Users\me\Documents\my-docs\zensical.toml
```

所有项目路径都以 `zensical.toml` 所在目录为根目录解析。

## 项目配置

`zensical.toml` 的基本结构如下：

```toml
[project]
site_name = "My Documentation Site"
docs_dir = "docs"
site_dir = "site"
use_directory_urls = true
site_url = "https://example.com"
```

### 配置字段

| 字段 | 是否必需 | 默认值 | 作用 |
| --- | --- | --- | --- |
| `site_name` | 是 | 无 | 站点名称，显示在侧栏和页面标题中 |
| `docs_dir` | 否 | `docs` | Markdown 和静态资源所在目录 |
| `site_dir` | 否 | `site` | 静态网站输出目录 |
| `use_directory_urls` | 否 | `true` | 是否生成目录风格 URL |
| `site_url` | 否 | 无 | 用于生成 canonical URL 的站点地址 |
| `nav` | 否 | 自动导航 | 显式定义导航标题、层级和顺序 |

`docs_dir` 和 `site_dir` 必须是相对于 `zensical.toml` 的相对路径，不能使用绝对路径或 `..`，并且两者不能相同。

### URL 风格

当 `use_directory_urls = true` 时：

```text
docs/guide/setup.md -> site/guide/setup/index.html -> /guide/setup/
```

当 `use_directory_urls = false` 时：

```text
docs/guide/setup.md -> site/guide/setup.html -> /guide/setup.html
```

`index.md` 和 `README.md` 会作为所在目录的首页。

### 导航

不配置 `nav` 时，Rustring 根据 `docs/` 的目录结构自动生成导航。页面和目录顺序可以通过 Front Matter 中的 `order` 调整。

显式导航示例：

```toml
[project]
site_name = "My Documentation Site"

nav = [
  { title = "Home", path = "index.md" },
  { title = "Guide", children = [
    { title = "Overview", path = "guide/index.md" },
    { title = "Setup", path = "guide/setup.md" },
    { title = "Resources", path = "guide/resources.md" }
  ] }
]
```

注意：

- `path` 相对于 `docs_dir`。
- 一个导航项必须定义 `path` 或 `children`，不能同时定义两者。
- 显式导航中的标题和顺序优先于 Front Matter。
- 显式导航引用不存在的 Markdown 文件时，构建会失败。

## 编写 Markdown

把 `.md` 文件放在 `docs_dir` 的任意子目录中。普通 Markdown 文件即使没有 Front Matter 也可以正常构建。

### Front Matter

每个 Markdown 文件可以在顶部添加 YAML Front Matter：

```md
---
title: 安装指南
summary: 安装并启动文档站点。
tags:
  - guide
  - setup
date: 2026-06-16
order: 2
---

# 安装指南

这里是正文。
```

支持字段：

| 字段 | 作用 |
| --- | --- |
| `title` | 页面标题，优先于 Markdown 中的第一个 H1 |
| `summary` | 页面摘要、HTML 描述和搜索摘要 |
| `tags` | 页面标签，用于标签展示、归档和知识图谱 |
| `date` | 页面日期，用于日期归档；推荐 `YYYY-MM-DD` |
| `order` | 自动导航中的同级排序，数字较小的排在前面 |

目录中的 `index.md` 或 `README.md` 所设置的 `order` 会控制整个目录分组在父级导航中的位置。没有 `order` 的项目排在有 `order` 的项目之后，再按稳定路径排序。

### 代码块

使用标准 Markdown 围栏代码块，并在开头标注语言：

````md
```rust
fn main() {
    println!("Hello, world!");
}
```
````

生成的页面会显示语言标签、轻量语法高亮和 `Copy` 按钮。未指定语言的代码块仍可复制。

### 数学公式

行内公式：

```md
当 $x > 0$ 时，结果为正数。
```

块级公式：

```md
$$
Y = \frac{A + B}{2}
$$
```

公式由页面中的 MathJax 脚本渲染。行内代码中的 `$...$` 不会被当作公式处理。

### 图片与静态资源

除 `.md` 以外的文件都会作为静态资源复制到输出目录，并保持相对于 `docs_dir` 的路径。

例如：

```text
docs/assets/logo.png -> site/assets/logo.png
```

在 `docs/index.md` 中引用：

```md
![Logo](assets/logo.png)
```

在 `docs/guide/setup.md` 中引用：

```md
![Logo](../assets/logo.png)
```

普通文件链接的写法相同：

```md
[下载示例文件](../assets/example.zip)
```

### 自定义字体

将 `.woff2`、`.woff`、`.ttf` 或 `.otf` 文件放入：

```text
docs/assets/fonts/
```

构建后，字体会复制到 `site/assets/fonts/`，并自动出现在页面侧栏的字体选择器中。页面还内置 `Sans`、`Serif` 和 `Mono` 字体选项。选择会保存在浏览器本地存储中。

## 站点功能

### 全文搜索

构建时会生成 `site/search.json`。搜索范围包括：

- 页面标题
- Markdown 标题
- 正文段落
- 列表项
- 代码块

结果按文档分组，并列出具体命中片段。点击结果会跳转到对应标题或正文块，同时高亮关键词。点击正文区域可取消高亮。

请通过 `rustring serve` 或正常的 Web 服务器预览搜索功能。直接使用 `file://` 打开 HTML 时，浏览器可能阻止读取 `search.json`。

### 主题与字体

侧栏提供以下主题：

- `Light`
- `Dark`
- `System`
- `Sepia`
- `Ocean`
- `Forest`

`System` 跟随操作系统的颜色偏好。手动选择的主题和字体会保存在浏览器本地存储中，刷新后仍然有效。

### 归档

每次构建都会生成：

```text
site/archive/index.html
site/archive/tags/index.html
```

- 日期归档读取 Front Matter 的 `date`。
- 标签归档读取 Front Matter 的 `tags`。
- 没有日期的页面不会出现在日期分组中。
- 没有标签的页面不会出现在标签分组中。

### 知识图谱

每次构建都会生成：

```text
site/graph.json
site/knowledge-graph/index.html
```

知识图谱根据以下关系连接页面：

- Markdown 文档之间的站内链接
- 页面与 Front Matter 标签的关系
- 拥有共同标签的页面

图谱页面支持搜索、拖拽节点、平移、缩放、显示或隐藏标签节点，以及调整向心力、节点排斥力、连接吸引力和连线长度。点击文档节点会进入对应页面。

D3 已随 Rustring 一起提供，图谱页面不依赖运行时 CDN。

## 常用命令

```bash
# 初始化当前目录
rustring init

# 使用其他配置文件初始化
rustring init --config path/to/zensical.toml

# 构建站点
rustring build

# 使用指定配置构建
rustring build --config path/to/zensical.toml

# 启动本地预览
rustring serve

# 指定配置和监听地址
rustring serve --config path/to/zensical.toml --addr 127.0.0.1:4000

# 查看版本与帮助
rustring --version
rustring --help
```

## 常见问题

### 修改了 `site/`，为什么下次构建后消失了？

`site/` 是生成目录，构建时会替换。请修改 `docs/` 中的内容或项目配置。

### 新页面为什么没有出现在导航中？

- 使用自动导航时，确认文件位于 `docs_dir` 中并重新构建。
- 使用显式 `nav` 时，还需要把页面加入 `zensical.toml` 的 `nav`。

### 页面标题为什么和第一个 H1 不一样？

Front Matter 中的 `title` 优先级更高。删除或修改该字段即可。

### 搜索框为什么没有结果？

确认已经成功执行 `build` 或 `serve`，并检查输出目录中是否存在 `search.json`。不要用 `file://` 直接测试搜索。

### 配置文件不在当前目录怎么办？

通过 `--config` 指定其完整路径。`docs_dir` 和 `site_dir` 仍相对于该配置文件所在目录解析。

### 构建失败后原有网站会丢失吗？

不会。Rustring 先在临时目录构建，只有构建成功后才替换原有输出目录；失败时保留上一次成功的站点。

## 当前限制

- 目前没有 `pip install`、Homebrew、预编译安装包或自动 GitHub Release；用户必须先安装 Rust/Cargo，再从 GitHub 编译安装。
- 安装过程需要网络连接，并可能需要本机 C/C++ 构建工具链。
- `docs_dir` 和 `site_dir` 只能使用相对于 `zensical.toml` 的安全相对路径，不能使用绝对路径或 `..`。
- 当前预览服务器主要用于本地开发，不应直接作为生产环境服务器。
- 搜索和知识图谱需要通过 HTTP 访问，直接打开本地 HTML 文件可能受到浏览器安全策略限制。
- 项目名称和命令名称未来可能调整；当前命令为 `rustring`，Rust package 名称为 `minizensical`。

## 项目背景

Rustring 起源于 Rust 程序设计课程项目，设计目标是用较小、清晰的 Rust 代码实现完整的 Markdown 静态站点构建流程。现在它可以作为独立 CLI 安装，并处理位于任意项目目录中的用户文档。

## License

仓库中随附的 D3 文件遵循其对应的 BSD 3-Clause License，详见 `vendor/d3/LICENSE`。项目其他部分的许可证请以仓库后续发布的正式许可文件为准。
