# Rustring 开发者文档

## 项目定位

Rustring 是一个用 Rust 编写的静态文档站点生成器。当前 Cargo 包名是 `minizensical`，安装后的命令行二进制名称是 `rustring`。

核心流程如下：

```text
zensical.toml + docs/ -> rustring build/serve -> site/
```

用户通常会在自己的文档项目中运行 `rustring`，不要假设用户文档一定放在本仓库内。本仓库中的 `docs/` 主要用于示例、测试和开发验证。

## 开发前检查

开始修改前应先执行：

```bash
git status --short --branch
```

保留所有无关的用户改动，不要因为实现当前任务而回滚他人或用户的未提交内容。

修改代码前至少阅读：

- `Cargo.toml`
- `src/main.rs`
- 与当前任务直接相关的模块

`README.md` 是面向用户的使用手册，`AGENTS.md` 和本文档是面向维护者的开发上下文。不要编辑 `site/` 下的生成文件；需要改变站点内容或构建结果时，应修改 `docs/`、`src/` 或 `zensical.toml`。

如果辅助脚本需要 Python 包，优先使用 `uv run --with <package> ...`，不要直接依赖全局 `pip` 或 `conda`。

## 模块职责

- `src/main.rs`：Clap CLI 入口，定义 `init`、`build`、`serve` 命令。
- `src/config.rs`：加载并验证 `zensical.toml`，所有项目路径以配置文件所在目录为根。
- `src/init.rs`：创建默认配置和 `docs/index.md`，不能覆盖已有文件。
- `src/scanner.rs`：递归扫描 `docs_dir`，区分 Markdown 源文件和需要复制的静态资源。
- `src/markdown.rs`：解析 YAML Front Matter、Markdown、数学公式、标题、搜索块和文档链接。
- `src/page.rs`：生成页面模型、输出路径、元数据、搜索数据和 canonical URL。
- `src/nav.rs`：生成自动导航或显式导航，并计算上一页/下一页顺序。
- `src/search.rs`：生成静态搜索索引。
- `src/graph.rs`：生成文档节点、标签节点和知识图谱关系。
- `src/render.rs`：维护 HTML 模板、CSS 和嵌入式浏览器脚本。
- `src/build.rs`：编排原子化站点构建、归档页、图谱输出、页面渲染和资源复制。
- `src/server.rs`：预览 HTTP 服务、文件监听、重建和 live reload。
- `tests/build.rs`：端到端构建行为和生成结果断言。
- `vendor/d3/`：本地 vendored D3 运行时代码及许可证，知识图谱必须保持离线可用。

## 关键行为约束

- `Config::root_dir` 必须是所选 `zensical.toml` 所在目录。
- `docs_dir` 和 `site_dir` 必须是 `root_dir` 下的安全相对路径。
- 拒绝绝对路径和 `..` 父级穿越。
- `docs_dir` 和 `site_dir` 不能相同。
- `index.md` 和 `README.md` 映射为所在目录的首页。
- 开启目录 URL 时，普通 Markdown 文件输出为 `<name>/index.html`。
- 关闭目录 URL 时，普通 Markdown 文件输出为 `<name>.html`。
- 构建失败时必须保留上一次成功的 `site_dir`，不要破坏 staging/backup 替换流程。
- `serve` 必须在文件变化时重新加载配置和源树，构建失败后继续服务上一次成功构建的站点。
- 搜索、归档、主题、字体、代码工具、数学公式和知识图谱都是公开用户功能，修改共享渲染或页面模型时要避免回归。
- 知识图谱使用本地 vendored D3，不要引入运行时 CDN 依赖。

## 内容契约

支持的 Front Matter 字段为：

```yaml
title: Optional page title
summary: Optional description and search excerpt
tags: [optional, list]
date: 2026-06-16
order: 1
```

字段含义：

- `title` 覆盖 Markdown 中第一个 H1 作为页面标题。
- `summary` 用作页面描述和搜索摘要。
- `tags` 用于标签归档和知识图谱。
- `date` 用于日期归档。
- `order` 只影响自动导航；目录首页的 `order` 会影响目录分组顺序。

显式导航项只能定义 `path` 或 `children` 之一，不能同时定义。`path` 相对于 `docs_dir`，引用不存在或重复的页面都应构建失败。

`docs_dir` 下的非 Markdown 文件会复制到相同的相对输出路径。`docs/assets/fonts/` 下的字体文件还会额外暴露给生成站点中的字体选择器。

## 开发原则

- 优先沿用现有模块边界和直接可读的 Rust 代码。
- 能用 TOML、YAML、Markdown 解析器处理结构化数据时，不要用脆弱的字符串拼接或匹配替代。
- 保持变更范围小，避免无关格式化、大规模重构或跨模块整理。
- 面向公共行为、路径处理、生成文件和失败恢复的改动，应添加或更新聚焦测试。
- 不要回滚无关的未提交改动，除非用户明确要求。
- 修改公开命令、配置、Front Matter、输出布局或用户可见行为时，同步更新 `README.md`。
- 开发文档应记录稳定规则，不要写临时任务日志或按时间排列的实现记录。

## 常用验证

普通 Rust 改动至少运行：

```bash
cargo test
cargo build --release
```

涉及 CLI、配置、构建、服务、路径或打包行为时，还应在仓库外创建临时目录测试真实 CLI：

```bash
tmpdir=$(mktemp -d)
cd "$tmpdir"
/absolute/path/to/repo/target/release/rustring init
/absolute/path/to/repo/target/release/rustring build
/absolute/path/to/repo/target/release/rustring serve
```

需要确认生成：

- `zensical.toml`
- `docs/index.md`
- `site/index.html`
- `site/search.json`
- `site/graph.json`
- `site/knowledge-graph/index.html`

如果修改 server 行为，还应通过 HTTP 请求首页和知识图谱页面。

文档-only 改动也应对照当前 CLI help、配置校验、页面元数据类型和相关测试，确认没有写入过期说明。收尾前运行：

```bash
git diff --check
```

## 发布和分发状态

当前支持的安装方式是：

```bash
cargo install --git https://github.com/TZMydl2006/Rustring.git --bin rustring
```

目前没有 PyPI 包、Homebrew formula、预构建二进制 release 或自动化跨平台发布流水线。在这些渠道真正存在并经过测试前，不要把它们写进用户文档或开发说明。
