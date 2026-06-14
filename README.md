# MiniZensical 用户指导

## 1. 这个项目现在是什么

`MiniZensical` 是一个用 Rust 写的静态站点生成器，灵感来自 zensical。

它保留了最核心的主链路：

```text
zensical.toml + docs/ -> site/
```

但现在已经不只是第一阶段的最小版了。当前版本额外加入了：

- 本地预览服务器 `serve`
- 自动重建
- 浏览器自动刷新
- YAML front matter
- 页面标签和摘要
- 页面归档（按日期 / 按标签）
- 前端全文搜索
- 多主题切换
- 代码块语法高亮与一键复制
- 页面字体切换，并能自动使用 `docs/assets/fonts/` 下提供的字体文件
- 更适合课程展示的页面样式

如果你要用一句话理解现在的项目，可以这样记：

> 它是一个“保留 zensical 核心流程、但更适合课程答辩展示”的 Rust 文档站生成器。

## 2. 第一次接触时，先认这几个目录

```text
minizensical/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── zensical.toml
├── docs/
├── site/
├── src/
├── tests/
└── target/
```

它们的作用分别是：

- `Cargo.toml`
  Rust 项目的配置文件，定义项目名、版本和依赖。

- `Cargo.lock`
  锁定依赖版本，保证不同机器上的编译结果更稳定。

- `README.md`
  就是这份用户指导。只要项目结构、功能、命令或用法发生变化，这份文档也必须一起更新。

- `zensical.toml`
  项目的用户配置文件。站点标题、输入目录、输出目录、URL 风格都在这里配置。

- `docs/`
  文档源目录。Markdown 页面、图片、PDF、其他静态资源都放这里。

- `site/`
  生成结果目录。每次构建都会重建，所以不要手动改里面的 HTML/CSS。

- `src/`
  Rust 源码目录。项目核心逻辑都在这里。

- `tests/`
  集成测试目录，用来验证构建、导航、front matter、搜索索引等核心功能。

- `target/`
  Cargo 编译缓存目录，不是日常阅读重点。

## 3. `src/` 里每个文件是干什么的

### `src/main.rs`

命令行入口。

它负责：

- 解析 `build` 和 `serve` 子命令
- 读取 `--config`
- 调用 `Config::load(...)`
- 进入构建流程或本地预览流程

### `src/lib.rs`

库入口。

它把主要模块统一导出，方便内部引用，也方便以后把项目当成 Rust 库复用。

### `src/error.rs`

统一错误类型。

它负责把这些错误变成更清楚的信息：

- 文件读写失败
- 配置文件解析失败
- front matter 解析失败
- 搜索索引序列化失败
- 模板渲染失败

### `src/config.rs`

配置读取与校验模块。

它负责：

- 解析 `zensical.toml`
- 提供默认值
- 校验 `docs_dir` 和 `site_dir`
- 校验显式 `nav` 的结构是否合法
- 生成 `docs_dir()` 和 `site_dir()` 等辅助方法

如果以后你们要新增全局配置字段，通常从这里改。

### `src/scanner.rs`

扫描 `docs/` 的模块。

它负责：

- 遍历 `docs/` 中所有文件
- 区分 Markdown 和静态资源
- 为后续构建生成统一的源文件列表

### `src/markdown.rs`

Markdown 解析模块。

它负责：

- 解析 YAML front matter
- 把 Markdown 转成 HTML
- 提取标题
- 生成页内目录 TOC
- 提取纯文本内容，供搜索索引使用
- 给标题生成锚点 id

这是“文本内容 -> 页面数据”的关键一步。

### `src/page.rs`

页面模型模块。

它负责定义一个页面最终需要有哪些信息，例如：

- 源文件路径
- 输出文件路径
- 页面标题
- HTML 正文
- TOC
- 标签
- 摘要
- 排序字段 `order`
- 日期字段 `date`
- 搜索摘要和纯文本内容

这里还负责输出路径规则，例如：

- `index.md -> index.html`
- `guide/setup.md -> guide/setup/index.html`
- `guide/setup.md -> guide/setup.html`

### `src/nav.rs`

导航模块。

它负责：

- 自动导航
- 显式导航
- 当前页面 active 状态
- 上一页 / 下一页链接
- 页面相对路径计算
- 自动导航下基于 `order` 的页面和目录分组排序

注意：

- 如果你配置了显式 `nav`，顺序以 `nav` 为准
- 如果你不写 `nav`，自动导航会参考 front matter 里的 `order`
- 目录分组的位置可以由该目录的 `index.md` 或 `README.md` 的 `order` 控制

### `src/render.rs`

HTML 渲染模块。

它负责：

- 内置页面模板
- 内置 CSS
- 内置主题切换脚本
- 内置前端搜索脚本
- 内置代码块增强脚本（语法高亮、复制按钮、字体切换）
- 归档页模板与归档页渲染函数（按日期 / 按标签）
- 把导航、TOC、正文、标签、摘要、搜索入口、主题切换入口、字体切换入口注入页面

如果你们要改答辩展示效果、页面布局、视觉风格，这里最关键。

### `src/search.rs`

搜索索引模块。

它负责：

- 从所有页面生成 `search.json`
- 把页面标题、Markdown 标题、正文段落 / 列表项 / 代码块整理成可跳转的搜索块

搜索是纯前端实现的，所以 Rust 这边只负责生成索引，不提供后端 API。

### `src/build.rs`

总调度模块。

它是整个项目的“总装配线”。

它按顺序完成：

1. 准备 staging 构建目录
2. 扫描 `docs/`
3. 生成所有 `Page`
4. 构建导航
5. 写入内置 CSS、主题脚本、搜索脚本和代码增强脚本
6. 构建归档数据（按日期 / 按标签）
7. 渲染归档页面 `archive/index.html` 与 `archive/tags/index.html`
8. 生成 `search.json`
9. 渲染所有文档页 HTML
10. 复制静态资源
11. 安全切换到最终 `site/`

### `src/server.rs`

本地预览服务器模块。

它负责：

- 启动前先做一次构建
- 提供本地 HTTP 预览
- 监听 `docs/` 和 `zensical.toml` 变化
- 自动重建
- 自动刷新浏览器页面
- 支持目录 URL、普通 `.html` 和中文文件名资源

## 4. `docs/` 和 `site/` 的关系

### `docs/`

输入目录。

当前示例里，你会看到这些内容：

- `docs/index.md`
- `docs/project-showcase.md`
- `docs/guide/index.md`
- `docs/guide/setup.md`
- `docs/guide/front-matter.md`
- `docs/guide/resources.md`
- `docs/assets/交大校徽-蓝色.png`

### `site/`

输出目录。

构建后会出现类似：

- `site/index.html`
- `site/project-showcase/index.html`
- `site/guide/index.html`
- `site/guide/setup/index.html`
- `site/guide/front-matter/index.html`
- `site/archive/index.html`
- `site/archive/tags/index.html`
- `site/search.json`
- `site/assets/minizensical.css`
- `site/assets/minizensical-theme.js`
- `site/assets/minizensical-search.js`
- `site/assets/minizensical-code.js`
- `site/assets/minizensical-math.js`
- `site/assets/交大校徽-蓝色.png`

一句话总结：

- 改内容，看 `docs/`
- 改逻辑，看 `src/`
- 看结果，看 `site/`

## 5. 现在已经支持什么

当前版本支持：

- 读取 `zensical.toml`
- 扫描 `docs/`
- Markdown 转 HTML
- 自动提取 H1
- YAML front matter
- 页面摘要和标签
- 页面排序字段 `order`
- 页面日期字段 `date`
- 页面归档（按日期页 / 按标签页）
- 自动导航
- 显式导航
- 上一页 / 下一页链接
- 页内目录
- 复制静态资源
- Markdown 本地图片路径自动重定位
- Markdown 行内公式与块级公式渲染
- 生成 `search.json`
- 前端即时搜索
- 日 / 夜间 / 跟随系统主题切换
- 代码块语法高亮
- 代码块一键复制
- 页面阅读字体切换
- 自动识别 `docs/assets/fonts/` 中的 `.woff2` / `.woff` / `.ttf` / `.otf` 字体文件
- 本地预览服务器
- 自动重建
- 浏览器自动刷新

## 5.1 本次增量：页面归档实现说明

这次更新新增了两个归档入口：

- `site/archive/index.html`：按日期归档（按年分组，组内按月份分组）
- `site/archive/tags/index.html`：按标签归档（无标签页面归入 `(untagged)`）

与功能直接相关的代码更新如下：

- `src/page.rs`
  `PageMetadata` 新增 `date: Option<String>`，用于承接 front matter 日期元数据。
- `src/build.rs`
  新增 `build_date_archive(...)`、`build_tag_archive(...)`、`write_archive_page(...)`，并在构建流水线中插入归档页渲染；同时向导航追加 `Archive -> By Date / By Tags`。
- `src/render.rs`
  新增 `ArchiveSection`/`ArchiveGroup` 数据结构、`render_archive_index(...)`、`render_tag_archive(...)`、`ARCHIVE_TEMPLATE` 以及归档页面样式。
- `src/nav.rs`
  `NavItem::section(...)` 与 `NavItem::page(...)` 改为 `pub`，用于在构建阶段组合归档导航项。
- `docs/*.md`
  示例文档 front matter 已补充 `date` 字段，用于驱动“按日期归档”。

## 5.2 本次增量：搜索定位功能实现说明

本次更新把原来的页面级搜索改成了更接近 Obsidian 的“命中片段定位”：

### 搜索面板简化
- 删除 `All / Title / Tags` 搜索模式选择，搜索固定匹配页面标题、Markdown 标题和正文内容。
- 删除高亮颜色选择器，关键词高亮固定使用一种颜色。
- 搜索结果不再显示匹配类型标签和 tag chip，界面只保留文档标题与命中片段。

### 按文档分组的命中片段
- `search.json` 中每个页面包含多个 `blocks`，包括页面标题、Markdown 标题、段落、列表项和代码块。
- 搜索结果先按文档分组，再列出该文档下被命中的片段。
- 每个片段内会直接高亮关键词，方便判断具体匹配位置。

### 精确跳转与页面内高亮
- 点击某个搜索片段会跳转到 `目标页面?mz-search=关键词#命中块id`。
- 目标页面加载后会滚动到对应标题或正文块，并只高亮该块里的关键词。
- 用户在正文区域点击后，页面内定位高亮会取消，同时 URL 中的 `mz-search` 参数会被移除。

### 与功能直接相关的代码更新
- `src/markdown.rs` 为可搜索正文块生成稳定 id，并把这些块写入 `search_blocks`。
- `src/search.rs` 输出新的 `blocks` 索引结构，不再把 tags、summary、整页 body 作为独立字段写入搜索索引。
- `src/render.rs` 简化搜索面板，重写前端搜索脚本与命中高亮样式。

## 5.3 本次增量：代码块高亮、复制与字体切换实现说明

本次更新在不改变原有构建主链路的前提下，新增了三个前端阅读体验增强：

### 代码块语法高亮
- Markdown 中使用围栏代码块时，如果写了语言标记，例如 ```` ```rust ````、```` ```bash ````、```` ```toml ````，生成页面会保留 `language-*` 类。
- 浏览器加载页面后，`minizensical-code.js` 会扫描 `.page-body pre > code`，根据语言类型给关键字、字符串、注释、数字、函数名、运算符等 token 添加高亮样式。
- 当前实现是轻量前端高亮，不引入新的 Rust 解析依赖，保持构建链路简单；未识别的语言会以普通代码块展示。

### 代码块一键复制
- 每个代码块会被增强成带工具栏的 `.code-block`。
- 工具栏左侧显示语言名，右侧显示 `Copy` 按钮。
- 点击 `Copy` 会复制原始代码文本，而不是复制高亮后的 HTML；复制成功后按钮短暂显示 `Copied`。
- 在不支持 `navigator.clipboard` 的环境中，会回退到临时 `textarea` 的复制方式。

### 字体切换
- 侧边栏新增 `Font` 面板，内置 `Sans`、`Serif`、`Mono` 三种字体栈。
- 用户选择会写入 CSS 变量 `--content-font`，并保存到浏览器本地存储 `minizensical-font-choice`，刷新后仍然保留。
- 如果把字体文件放到 `docs/assets/fonts/` 下，支持 `.woff2`、`.woff`、`.ttf`、`.otf`，构建时会自动识别并加入字体切换按钮。
- 这些字体会像其他静态资源一样复制到 `site/assets/fonts/`，同时在 `site/assets/minizensical.css` 顶部生成对应的 `@font-face`。

### 与功能直接相关的代码更新
- `src/render.rs`：
  - 新增 `FontOption`、`default_font_options(...)`、`stylesheet_contents(font_options)`，用于生成字体按钮和动态 `@font-face`。
  - `MAIN_TEMPLATE` 与 `ARCHIVE_TEMPLATE` 新增 `Font` 面板，并加载 `minizensical-code.js`。
  - `STYLE_SHEET` 新增 `.font-panel`、`.font-option`、`.code-block`、`.code-toolbar`、`.copy-code-button`、`.token-*` 等样式。
  - 新增 `CODE_SCRIPT`，统一处理字体切换、代码块高亮和复制按钮。
- `src/build.rs`：
  - 新增 `build_font_options(...)` 和 `is_provided_font_asset(...)`，从 `docs/assets/fonts/` 自动收集提供的字体文件。
  - `write_theme_assets(...)` 改为接收字体选项，写出动态 CSS，并新增 `site/assets/minizensical-code.js`。
  - 渲染普通页面和归档页时都传入同一组字体选项，保证全站行为一致。
- `tests/build.rs`：
  - 在集成测试中加入示例代码块和占位字体文件。
  - 校验生成页面包含 `minizensical-code.js`、字体切换入口、自动识别的字体按钮，以及 CSS 中的 `@font-face`。

## 5.4 本次增量：Markdown 图片路径重定位实现说明

本次更新让 Markdown 中的本地图片路径可以按照源文件位置书写，不再要求用户手工计算生成页面的 URL 深度：

- 支持 `![alt](./image.png)`、`![alt](../assets/image.png)`、`![alt](assets/image.png)` 等常见本地图片写法。
- 构建时先根据 Markdown 文件所在目录解析图片位置，再根据生成 HTML 所在目录计算最终相对路径。
- `http://`、`https://` 等外部图片 URL 保持原样，不参与重写。
- 非 Markdown 静态资源仍然按原有规则复制到 `site/` 的对应相对路径。
- 普通文件链接不会自动重写，本次更新只处理嵌入图片。

与功能直接相关的代码更新如下：

- `src/markdown.rs`
  在 Markdown 转 HTML 前重写本地图片事件中的目标路径，并保留外部 URL。
- `src/page.rs`
  在渲染 Markdown 时传入源文件相对路径和输出页面路径，为图片重定位提供上下文。
- `tests/build.rs`
  新增图片路径集成测试，覆盖根目录图片、子目录同级图片、上级 assets 图片、无 `./` 前缀图片和外部 URL。
- `docs/guide/resources.md`
  更新资源使用说明，统一要求嵌入图片路径相对于 Markdown 源文件书写。

## 5.5 本次增量：Markdown 公式渲染实现说明

本次更新增加了常用 LaTeX 公式渲染能力：

- 行内公式使用 `$...$`，例如 `$V_{GS} > V_T$`。
- 块级公式使用 `$$...$$`，例如 `$$ Y = \frac{A + B}{2} $$`。
- 独立成行的 `$$` 支持包裹多行公式和内部空行；构建时会清理多余空白，并把普通多行内容包装为 `gathered` 环境逐行展示。
- 行内公式会清理公式内部的首尾空白；中文文本、中文标点和括号邻接公式时，也会移除多余间隔，同时保留英文单词之间必要的空格。
- 行内公式只增加防换行规则，不覆盖 MathJax SVG 自带的宽度和基线偏移，避免 CSS 冲突导致短公式出现空隙或与正文错位。
- 行内 MathJax 容器使用居中对齐，使公式字形在浏览器分配的容器内部保持视觉居中。
- 构建时由 `pulldown-cmark` 识别公式语法，生成 `.math-inline` 和 `.math-display` 容器。
- 浏览器加载页面后，`minizensical-math.js` 会在页面包含公式时按需加载 MathJax，并将公式渲染为 SVG。
- 行内代码和代码块中的 `$...$` 会保持原样，不会被误识别为公式。
- 公式文本仍会进入页面纯文本和搜索块，避免搜索能力回退。

常见写法示例：

```md
栅源电压为 $V_{GS}$。

$$
Y = \overline{AB}
$$
```

需要按等号对齐等精细排版时，可以显式使用 LaTeX 的 `aligned` 环境和 `\\`：

```md
$$
\begin{aligned}
Y_0 &= \overline{A_3} \overline{A_2} A_1 + A_3 \\
Y_1 &= \overline{A_3} A_2 + A_3
\end{aligned}
$$
```

与功能直接相关的代码更新如下：

- `src/markdown.rs`
  启用 `Options::ENABLE_MATH`，并在纯文本和搜索块提取时保留公式内容。
- `src/render.rs`
  新增公式样式、`minizensical-math.js` 和页面脚本入口。
- `src/build.rs`
  构建时生成 `site/assets/minizensical-math.js`，并为普通页面注入正确的相对路径。
- `tests/build.rs`
  增加公式构建测试，覆盖行内公式、块级公式、代码保护和 MathJax 脚本生成。

注意：公式排版依赖浏览器访问 MathJax CDN。页面在离线环境下仍会显示原始公式文本，但不会转换为 SVG 排版结果。

## 6. 现在还没有做什么

为了保持课程项目范围可控，目前还没有做：

- `mkdocs.yml` 兼容
- Python 兼容层
- 插件系统
- 可配置的主题包系统
- 深度搜索排序优化
- 服务端搜索 API
- 作者信息系统（页面署名）
- 多维归档筛选（例如作者、年份与标签联动过滤）
- 复杂 taxonomy 自动页面生成

## 7. 如何快速跑起来

### 方式一：使用命令行工具（推荐）

如果你已经拿到了 `rustring.exe`，可以直接使用：

#### 步骤 1：初始化项目

```bash
rustring init
```

这会生成默认的 `zensical.toml` 和 `docs/index.md`。

#### 步骤 2：看一下默认配置

生成的 `zensical.toml` 内容如下：

```toml
[project]
site_name = "My Documentation Site"
docs_dir = "docs"
site_dir = "site"
use_directory_urls = true
```

你可以根据需要编辑它：
- `site_name`：站点名称，会显示在侧边栏和页面标题里
- `docs_dir`：文档源目录，默认 `docs`
- `site_dir`：构建输出目录，默认 `site`
- `use_directory_urls`：设为 `true` 时使用目录风格 URL（如 `guide/setup/`），`false` 时使用 `.html` 后缀
- `site_url`：可选，用于生成 canonical URL
- `nav`：可选，显式定义导航结构；不写则使用自动导航

#### 步骤 3：生成站点

```bash
rustring build
```

如果想手动指定配置文件：

```bash
rustring build --config zensical.toml
```

#### 步骤 4：本地预览

```bash
rustring serve
```

默认地址：

```text
http://127.0.0.1:3000
```

自定义地址：

```bash
rustring serve --addr 127.0.0.1:4000
```

#### 步骤 5：看生成结果

构建完成后可以直接打开：

```text
site/index.html
```

但如果你要体验搜索，推荐使用 `serve`，因为浏览器对本地 `file://` 的 JSON 读取限制比较多。

### 方式二：用 Cargo 开发运行

如果你有 Rust 开发环境，也可以从源码运行：

#### 步骤 1：进入项目目录

```bash
cd Rustring
```

#### 步骤 2：查看版本

```bash
rustring --version
```

#### 步骤 3：初始化项目（或直接用现有配置）

```bash
rustring init
```

#### 步骤 4：生成站点

```bash
cargo run -- build
```

或直接使用编译好的二进制：

```bash
target/release/rustring build
```

#### 步骤 5：本地预览

```bash
cargo run -- serve
```

#### 步骤 6：看生成结果

构建完成后可以直接打开：

```text
site/index.html
```

但如果你要体验搜索，推荐使用 `serve`，因为浏览器对本地 `file://` 的 JSON 读取限制比较多。

当前主题切换不依赖配置文件，页面侧边栏中的按钮会直接控制 `Day`、`Night` 和 `System` 三种模式。

## 8. `zensical.toml` 怎么写

### 基本配置

```toml
[project]
site_name = "MiniZensical Course Showcase"
docs_dir = "docs"
site_dir = "site"
use_directory_urls = true
site_url = "https://example.com"
```

### 字段说明

#### `site_name`

站点名称。显示在侧边栏品牌和页面标题里。

#### `docs_dir`

文档源目录，默认是 `docs`。

#### `site_dir`

输出目录，默认是 `site`。

#### `use_directory_urls`

控制输出链接风格。

如果是 `true`：

- `guide/setup.md -> site/guide/setup/index.html`

如果是 `false`：

- `guide/setup.md -> site/guide/setup.html`

#### `site_url`

可选。写了之后会生成 canonical URL。

#### `nav`

可选。

- 不写：自动导航，顺序由页面和目录 `index.md` / `README.md` 的 `order` 决定
- 写了：显式导航，顺序和标题以配置为准

显式导航示例：

```toml
nav = [
  { title = "Home", path = "index.md" },
  { title = "Guide", children = [
    { title = "Overview", path = "guide/index.md" },
    { title = "Resources", path = "guide/resources.md" }
  ] }
]
```

注意：

- 一个导航项不能同时写 `path` 和 `children`
- `path` 必须相对 `docs/`
- 如果显式 `nav` 和 front matter `order` 同时存在，导航顺序还是以 `nav` 为准
- 自动导航下，目录分组会读取该目录 `index.md` 或 `README.md` 的 `order`

## 9. Front Matter 怎么用

现在每个 Markdown 文件顶部都可以写一个可选的 YAML front matter：

```md
---
title: Front Matter
summary: Use metadata to control how the page is displayed and organized.
tags:
  - guide
  - metadata
date: 2025-01-20
order: 2
---
# 这里仍然可以有 H1
```

### 支持字段

- `title`
- `summary`
- `tags`
- `order`
- `date`

### 规则

- `title` 优先级高于 Markdown 里的 `# H1`
- `summary` 会显示在页面顶部，也会作为页面描述使用
- `tags` 会显示成标签，并用于按标签归档
- `order` 只在自动导航下生效，用来调整同级页面或目录分组顺序
- 如果 `order` 写在目录的 `index.md` 或 `README.md`，它会控制这个目录分组在父级导航里的位置
- 没有写 `order` 的页面或目录分组会排在写了 `order` 的项目后面，并继续按路径兜底排序
- `date` 用于归档页分组，推荐格式 `YYYY-MM-DD`（如 `2025-04-01`）

### 兼容性

旧的 Markdown 页面不写 front matter 也完全可以继续构建。

## 10. 搜索怎么工作

这是第二阶段最重要的新增功能之一。

### 构建时会发生什么

执行 `build` 或 `serve` 时，系统会：

1. 收集每个页面的标题
2. 收集 Markdown 中的各级标题
3. 为段落、列表项和代码块生成稳定的搜索块 id
4. 把这些内容写入 `site/search.json` 的 `blocks`

### 浏览器里会发生什么

页面加载后，前端搜索脚本会读取 `search.json`。

搜索框支持检索：

- 页面标题
- Markdown 标题
- 正文内容

搜索结果会按文档分组显示，每个文档下面列出具体命中的片段。点击片段后，浏览器会跳到对应页面的对应标题或正文块，并高亮该块里的关键词。

### 推荐演示搜索词

你们现在的示例站点里，适合答辩时演示的关键词有：

- `front matter`
- `architecture`
- `preview`
- `search`

### 页面内高亮取消

从搜索结果跳转到页面后，目标块会保持高亮。用户在正文区域点击任意位置后，高亮会取消，URL 中的 `mz-search` 参数也会被清理。

## 11. 主题切换怎么用

页面侧边栏里现在有三个按钮：

- `Day`
- `Night`
- `System`

规则如下：

- `Day` 强制使用浅色主题
- `Night` 强制使用深色主题
- `System` 跟随浏览器或操作系统的日夜偏好

主题选择会保存在浏览器本地存储里，所以刷新页面后仍然会保留上一次的手动选择。

## 11.1 字体切换和代码块工具怎么用

### 字体切换

页面侧边栏的 `Font` 面板提供：

- `Sans`：默认无衬线字体栈，适合正文阅读
- `Serif`：衬线字体栈，适合偏文档或论文风格的内容
- `Mono`：等宽字体栈，适合代码较多的页面
- 自动识别的提供字体：放在 `docs/assets/fonts/` 下的字体文件会在构建后自动出现在这里

例如你放入：

```text
docs/assets/fonts/my-display-font.woff2
```

构建后会得到：

```text
site/assets/fonts/my-display-font.woff2
```

同时页面会出现一个 `My Display Font` 按钮。点击后，正文会切换到该字体，并保存在浏览器本地存储中。

### 代码块高亮与复制

Markdown 中继续使用标准围栏代码块即可：

````md
```rust
fn main() {
    println!("Hello, world!");
}
```
````

构建后的页面会在浏览器中自动增强为：

- 带语言标签的代码块工具栏
- 按语言做轻量语法高亮
- `Copy` 按钮，一键复制原始代码文本

如果代码块没有写语言，仍然会显示复制按钮，但不会强行套用某一种语法规则。

## 12. 图片和其他资源怎么加

把资源放进 `docs/` 任意位置即可。

例如：

```text
docs/assets/交大校徽-蓝色.png
```

构建后它会变成：

```text
site/assets/交大校徽-蓝色.png
```

### 在根目录页面中引用

```md
![校徽](assets/交大校徽-蓝色.png)
```

### 在 `docs/guide/` 中引用

```md
![校徽](../assets/交大校徽-蓝色.png)
```

### 如果只想放一个链接

```md
[查看原图](../assets/交大校徽-蓝色.png)
```

示例可直接看：

- `docs/index.md`
- `docs/guide/resources.md`

## 13. 日常最常见的操作

### 新增一个页面

1. 在 `docs/` 下创建新的 `.md`
2. 需要排序时，加 front matter `order`
3. 如果使用显式 `nav`，再去 `zensical.toml` 补上它
4. 运行 `rustring serve`

### 修改页面标题

有两种方式：

1. 改 Markdown 的第一个 `# H1`
2. 用 front matter 里的 `title`

推荐第二种，因为更适合和搜索、摘要一起管理。

### 修改页面顺序

如果你使用自动导航，改 front matter：

```yaml
order: 2
```

如果你想调整整个目录分组的位置，把 `order` 写在这个目录的 `index.md` 或 `README.md` 里：

```yaml
---
title: Guide Overview
order: 3
---
```

如果你使用显式导航，去改 `zensical.toml` 的 `nav` 顺序。

### 本地预览

运行：

```bash
rustring serve
```

当 `serve` 正在运行时：

- 改 Markdown 会自动重建
- 改图片或其他资源会自动重建
- 改 `zensical.toml` 会自动重建
- 成功重建后，浏览器会自动刷新

### 切换主题演示

打开侧边栏中的 `Day` / `Night` / `System` 按钮即可。

如果你们要答辩演示，我建议：

1. 先用 `Day` 模式介绍首页
2. 再切到 `Night` 模式，展示项目已经支持多主题
3. 最后把它切回 `System`，说明它能跟随系统偏好

## 14. 如果你第一次读代码，建议顺序

推荐阅读顺序：

1. `src/main.rs`
   先看命令入口

2. `src/build.rs`
   看整体流水线

3. `src/config.rs`
   看配置是怎么进入系统的

4. `src/scanner.rs`
   看输入文件怎么被识别

5. `src/markdown.rs`
   看 Markdown 和 front matter 是怎么解析的

6. `src/page.rs`
   看页面数据是怎么组织的

7. `src/nav.rs`
   看导航和顺序逻辑

8. `src/search.rs`
   看搜索索引怎么生成

9. `src/render.rs`
   看最终页面、样式、主题切换和前端搜索脚本

10. `src/server.rs`
    看预览服务器如何工作

## 15. 如果你们之后继续扩展，优先改哪里

### 想增强搜索

优先看：

- `src/search.rs`
- `src/render.rs`
- `src/build.rs`

### 想增加更多 front matter 字段

优先看：

- `src/markdown.rs`
- `src/page.rs`

### 想继续增强主题或字体系统

优先看：

- `src/render.rs`
- `src/build.rs`

### 想继续扩展归档功能

优先看：

- `src/build.rs`（归档分组与输出入口）
- `src/render.rs`（归档模板与样式）
- `src/page.rs`（归档元数据字段）

### 想继续提升答辩展示效果

优先看：

- `src/render.rs`
- `docs/index.md`
- `docs/project-showcase.md`

### 想改本地预览体验

优先看：

- `src/server.rs`
- `src/build.rs`

## 16. 常见问题

### 为什么我改了 `site/` 里的 HTML，但下次又没了？

因为 `site/` 是构建结果目录，每次构建都会重建。真正应该改的是：

- 文档内容：`docs/`
- 页面模板和样式：`src/render.rs`
- 配置：`zensical.toml`

### 为什么新增页面后左侧没显示？

可能是：

- 你使用的是显式 `nav`，但没有把新页面加进去
- 你还没重新构建

如果正在运行 `rustring serve`，保存文件后会自动重建。

### 为什么 `title` 和 Markdown 第一个标题不一样？

因为你写了 front matter `title`，它会覆盖 Markdown 的 H1。

### 为什么搜索框没有结果？

先检查：

- 页面是否真的进入了 `docs/`
- 有没有成功执行 `build` 或 `serve`
- 是否存在 `site/search.json`

另外，如果你直接用浏览器打开 `file://.../site/index.html`，部分浏览器会限制本地 JSON 读取。此时推荐用 `rustring serve` 预览。

### 为什么主题切换刷新后还能保留？

因为主题选择被保存到了浏览器本地存储中，这是当前设计的一部分。

## 17. 当前推荐的工作方式

如果你们四个人继续协作，我建议这样分工：

- 负责 CLI / 配置的人：看 `src/main.rs`、`src/config.rs`
- 负责 Markdown / front matter 的人：看 `src/markdown.rs`、`src/page.rs`
- 负责搜索 / 前端交互 / 主题的人：看 `src/search.rs`、`src/render.rs`
- 负责预览 / 集成 / 测试的人：看 `src/build.rs`、`src/server.rs`、`tests/build.rs`

## 18. 文档维护规则

从现在开始，`README.md` 视为项目的一部分。

后续只要发生下面这些变化，就必须同步更新这份文档：

- 新增命令
- 修改配置字段
- 新增或删除重要模块
- 修改 front matter 规则
- 修改搜索行为
- 修改主题切换行为
- 修改字体切换行为
- 修改代码块高亮或复制行为
- 修改目录结构
- 修改页面输出规则
- 改变推荐使用方式

简单说就是：

**代码怎么变，这份用户指导就怎么跟着变。**

和我合作开发的 AI agent 或者开发者请注意，你的代码应该遵守这些限制：

- 你的改动应该优先保持现有模块边界稳定，不要随意打散 `config / markdown / page / nav / render / build / server / search` 这条主框架
- 你不应该直接手改 `site/` 里的生成结果，应该修改 `docs/`、`src/` 或 `zensical.toml`
- 你不应该为了实现局部功能而破坏已有命令、构建链路、搜索、主题切换或预览服务器的工作方式
- 你在修改公开行为、配置字段、front matter 规则、主题行为或目录结构时，必须同步更新这份 `README.md`
- 你应该尽量做小步、可测试、可回退的改动，并在提交前至少运行相关测试
- 你不应该随意删除现有测试，除非同时提供覆盖相同行为的新测试
- 你在多人协作时应该尽量减少大范围无关重构，优先避免制造不必要的合并冲突

## 19. 主题扩展与知识图谱开发记录

本次增量开发在不改变原有 `zensical.toml + docs/ -> site/` 主流程的前提下，扩展了主题系统，并新增基于 Markdown 文档的知识图谱 MVP。相关功能全部仍然在静态站点构建阶段完成，不引入后端 API、图数据库、AI/NLP 服务或大型前端框架。

### 19.1 主题系统扩展

更新位置：

- `src/render.rs`

主要作用：

- 保留原有 `Light`、`Dark`、`System` 三种主题入口，并新增 `Sepia`、`Ocean`、`Forest` 三种展示型主题。
- 继续使用 CSS 变量统一管理主题颜色，不把背景色、正文色、边框色、强调色、代码块样式散落硬编码到多个位置。
- 新增并统一使用 `--control-border`、`--focus-ring`、`--item-border`、`--inline-code-bg`、`--quote-bg`、`--token-*` 等变量，让普通页面、归档页面、搜索结果、导航、引用块、行内代码和代码块都能随主题切换。
- 主题选择继续保存到浏览器 `localStorage`，刷新页面后保留选择；`System` 模式仍然跟随浏览器 `prefers-color-scheme`。
- 主题切换按钮文案从旧的 `Day` / `Night` 调整为更直接的 `Light` / `Dark`，并在普通页面、归档页面和知识图谱页面中保持一致。

### 19.2 Markdown 链接提取

更新位置：

- `src/markdown.rs`
- `src/page.rs`

主要作用：

- `RenderedMarkdown` 新增 `links: Vec<MarkdownLink>`，用于保存 Markdown 正文中的普通链接。
- `MarkdownLink` 包含 `destination` 和可选 `title`，供后续图谱构建解析文档引用关系。
- 链接提取通过 `pulldown-cmark` 的事件流完成，匹配 `Tag::Link` 事件，不使用脆弱的字符串正则匹配。
- `Page` 新增 `links` 字段，把 Markdown 解析阶段得到的链接带入页面模型，供 `src/graph.rs` 使用。

### 19.3 知识图谱数据生成

更新位置：

- `src/graph.rs`
- `src/lib.rs`
- `src/error.rs`
- `src/build.rs`

主要作用：

- 新增 `src/graph.rs` 模块，定义 `Graph`、`GraphNode`、`GraphEdge`、`build_knowledge_graph(...)` 和 `graph_json_path()`。
- 构建阶段新增输出 `site/graph.json`，JSON 由 `serde_json::to_vec_pretty` 生成，结构包含 `version`、`nodes` 和 `edges`。
- 图谱节点类型包括：
  - `document`：每篇 Markdown 页面生成一个文档节点，包含标题、URL、summary、tags、date、source。
  - `tag`：front matter 中出现的每个标签生成一个标签节点。
- 图谱边类型包括：
  - `has_tag`：文档指向标签，权重 `2.0`。
  - `links_to`：文档指向被 Markdown 链接引用的站内文档，权重 `3.0`。
  - `shared_tag`：共享标签的文档之间建立边，权重为共享标签数量乘以 `0.8`。
- 站内 Markdown 链接解析会跳过外链、根路径链接、不可解析路径和非 `.md` 目标，避免无效链接导致构建失败。
- `src/error.rs` 新增 `SerializeGraph` 错误类型，用于报告 `graph.json` 序列化失败。
- `src/lib.rs` 导出 `graph` 模块，让项目内部模块边界保持清晰。

### 19.4 Knowledge Graph 页面

更新位置：

- `src/render.rs`
- `src/build.rs`

主要作用：

- 构建阶段新增页面 `site/knowledge-graph/index.html`。
- 导航中在 `Archive` 后追加 `Knowledge Graph` 入口。
- 页面复用原有侧栏、搜索、主题切换和字体切换结构，保持现有视觉和交互风格。
- 新增内置前端资源 `site/assets/minizensical-graph.js`，由 `src/render.rs` 中的 `GRAPH_SCRIPT` 输出。
- 前端脚本通过 `fetch("../graph.json")` 读取图谱数据，并使用 D3 v7 的力导向模拟渲染节点和边。
- D3 7.9.0 随项目保存在 `vendor/d3/`，构建时输出为 `site/assets/d3.min.js`；图谱页面不依赖 CDN 或运行时外网连接。
- 图谱页面支持：
  - 拖拽节点、画布平移、滚轮或触控缩放，并在窗口尺寸变化时自动适配。
  - 默认只展示文档节点以及文档之间的引用和共享标签关系。
  - 通过 `Show tags` 开关按需加入可拖拽的标签节点和 `has_tag` 连线。
  - 搜索时突出命中的文档或标签，不触发整张图重新布局。
  - 调整图谱向心力、节点排斥力、相连节点吸引力和连线长度。
  - 悬停节点时突出直接邻居和连接边，弱化无关内容；放大画布后显示更多标签。
  - 点击 `document` 节点跳转到对应文档页面。
  - 点击 `tag` 节点突出与该标签关联的文档，再次点击或点击空白处取消。
  - 控制面板悬浮在图谱右上角，桌面端默认展开，窄屏默认折叠。

### 19.5 测试与验收覆盖

更新位置：

- `tests/build.rs`
- `src/markdown.rs`

主要作用：

- 集成测试新增对 `site/graph.json`、`site/knowledge-graph/index.html` 和 `site/assets/minizensical-graph.js` 的存在性检查。
- 集成测试验证图谱 JSON 只包含 `document`、`tag` 节点。
- 集成测试验证只生成 `has_tag`、`links_to`、`shared_tag` 三类边，并检查所有边的端点与节点类型匹配。
- Markdown 单元测试验证普通 Markdown 链接能通过 `pulldown-cmark` 事件流提取，且图片不会被误当成普通链接。
- 原有最小站点构建测试同步验证新增主题 `Sepia`、`Ocean`、`Forest`、图谱页面入口和图谱脚本资源。

验收时建议执行：

```bash
cargo test
rustring build
```

构建后重点检查：

- `site/graph.json` 存在且是合法 JSON。
- `site/knowledge-graph/index.html` 存在。
- `site/assets/minizensical-graph.js` 存在。
- 页面主题能切换到 `Light`、`Dark`、`System`、`Sepia`、`Ocean`、`Forest`。
- 通过本地 HTTP 预览打开知识图谱页面时，节点可拖拽，画布可平移缩放，四项力参数和 Tags 开关生效，document 节点能跳转到文档页。
