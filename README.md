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
- 前端全文搜索
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
- 自动导航下基于 `order` 的同级排序

注意：

- 如果你配置了显式 `nav`，顺序以 `nav` 为准
- 如果你不写 `nav`，自动导航会参考 front matter 里的 `order`

### `src/render.rs`

HTML 渲染模块。

它负责：

- 内置页面模板
- 内置 CSS
- 内置前端搜索脚本
- 把导航、TOC、正文、标签、摘要、搜索入口注入页面

如果你们要改答辩展示效果、页面布局、视觉风格，这里最关键。

### `src/search.rs`

搜索索引模块。

它负责：

- 从所有页面生成 `search.json`
- 把标题、摘要、标签、一级/二级标题、正文纯文本整理成搜索索引

搜索是纯前端实现的，所以 Rust 这边只负责生成索引，不提供后端 API。

### `src/build.rs`

总调度模块。

它是整个项目的“总装配线”。

它按顺序完成：

1. 准备 staging 构建目录
2. 扫描 `docs/`
3. 生成所有 `Page`
4. 构建导航
5. 写入内置 CSS 和搜索脚本
6. 生成 `search.json`
7. 渲染所有 HTML 页面
8. 复制静态资源
9. 安全切换到最终 `site/`

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
- `site/search.json`
- `site/assets/minizensical.css`
- `site/assets/minizensical-search.js`
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
- 自动导航
- 显式导航
- 上一页 / 下一页链接
- 页内目录
- 复制静态资源
- 生成 `search.json`
- 前端即时搜索
- 本地预览服务器
- 自动重建
- 浏览器自动刷新

## 6. 现在还没有做什么

为了保持课程项目范围可控，目前还没有做：

- `mkdocs.yml` 兼容
- Python 兼容层
- 插件系统
- 多主题切换
- 深度搜索排序优化
- 服务端搜索 API
- 作者 / 日期 / 归档系统
- 复杂 taxonomy 自动页面生成

## 7. 如何快速跑起来

### 步骤 1：进入项目目录

```bash
cd /Users/wangyilin/Downloads/西交/Rust程序设计/RustProject/minizensical
```

### 步骤 2：看一下默认配置

根目录的 `zensical.toml` 当前示例非常简单：

```toml
[project]
site_name = "MiniZensical Course Showcase"
docs_dir = "docs"
site_dir = "site"
use_directory_urls = true
```

这里没有写 `nav`，表示使用自动导航。

### 步骤 3：生成站点

```bash
cargo run -- build
```

如果想手动指定配置文件：

```bash
cargo run -- build --config zensical.toml
```

### 步骤 4：本地预览

```bash
cargo run -- serve
```

默认地址：

```text
http://127.0.0.1:3000
```

自定义地址：

```bash
cargo run -- serve --addr 127.0.0.1:4000
```

### 步骤 5：看生成结果

构建完成后可以直接打开：

```text
site/index.html
```

但如果你要体验搜索，推荐使用 `serve`，因为浏览器对本地 `file://` 的 JSON 读取限制比较多。

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

- 不写：自动导航，顺序由目录结构和 `order` 决定
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

## 9. Front Matter 怎么用

现在每个 Markdown 文件顶部都可以写一个可选的 YAML front matter：

```md
---
title: Front Matter
summary: Use metadata to control how the page is displayed and indexed.
tags:
  - guide
  - metadata
order: 2
---
# 这里仍然可以有 H1
```

### 支持字段

- `title`
- `summary`
- `tags`
- `order`

### 规则

- `title` 优先级高于 Markdown 里的 `# H1`
- `summary` 会显示在页面顶部，也会进入搜索结果摘要
- `tags` 会显示成标签，也会进入搜索索引
- `order` 只在自动导航下生效，用来调整同级页面顺序

### 兼容性

旧的 Markdown 页面不写 front matter 也完全可以继续构建。

## 10. 搜索怎么工作

这是第二阶段最重要的新增功能之一。

### 构建时会发生什么

执行 `build` 或 `serve` 时，系统会：

1. 收集每个页面的标题
2. 收集 `summary`
3. 收集 `tags`
4. 收集一级和二级标题
5. 收集正文纯文本
6. 生成 `site/search.json`

### 浏览器里会发生什么

页面加载后，前端搜索脚本会读取 `search.json`。

搜索框支持检索：

- 页面标题
- 页面摘要
- 标签
- 一级/二级标题
- 正文内容

### 推荐演示搜索词

你们现在的示例站点里，适合答辩时演示的关键词有：

- `front matter`
- `architecture`
- `preview`
- `search`

## 11. 图片和其他资源怎么加

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

## 12. 日常最常见的操作

### 新增一个页面

1. 在 `docs/` 下创建新的 `.md`
2. 需要排序时，加 front matter `order`
3. 如果使用显式 `nav`，再去 `zensical.toml` 补上它
4. 运行 `cargo run -- serve`

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

如果你使用显式导航，去改 `zensical.toml` 的 `nav` 顺序。

### 本地预览

运行：

```bash
cargo run -- serve
```

当 `serve` 正在运行时：

- 改 Markdown 会自动重建
- 改图片或其他资源会自动重建
- 改 `zensical.toml` 会自动重建
- 成功重建后，浏览器会自动刷新

## 13. 如果你第一次读代码，建议顺序

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
   看最终页面、样式和前端搜索脚本

10. `src/server.rs`
    看预览服务器如何工作

## 14. 如果你们之后继续扩展，优先改哪里

### 想增强搜索

优先看：

- `src/search.rs`
- `src/render.rs`
- `src/build.rs`

### 想增加更多 front matter 字段

优先看：

- `src/markdown.rs`
- `src/page.rs`

### 想继续提升答辩展示效果

优先看：

- `src/render.rs`
- `docs/index.md`
- `docs/project-showcase.md`

### 想改本地预览体验

优先看：

- `src/server.rs`
- `src/build.rs`

## 15. 常见问题

### 为什么我改了 `site/` 里的 HTML，但下次又没了？

因为 `site/` 是构建结果目录，每次构建都会重建。真正应该改的是：

- 文档内容：`docs/`
- 页面模板和样式：`src/render.rs`
- 配置：`zensical.toml`

### 为什么新增页面后左侧没显示？

可能是：

- 你使用的是显式 `nav`，但没有把新页面加进去
- 你还没重新构建

如果正在运行 `cargo run -- serve`，保存文件后会自动重建。

### 为什么 `title` 和 Markdown 第一个标题不一样？

因为你写了 front matter `title`，它会覆盖 Markdown 的 H1。

### 为什么搜索框没有结果？

先检查：

- 页面是否真的进入了 `docs/`
- 有没有成功执行 `build` 或 `serve`
- 是否存在 `site/search.json`

另外，如果你直接用浏览器打开 `file://.../site/index.html`，部分浏览器会限制本地 JSON 读取。此时推荐用 `cargo run -- serve` 预览。

## 16. 当前推荐的工作方式

如果你们四个人继续协作，我建议这样分工：

- 负责 CLI / 配置的人：看 `src/main.rs`、`src/config.rs`
- 负责 Markdown / front matter 的人：看 `src/markdown.rs`、`src/page.rs`
- 负责搜索 / 前端交互的人：看 `src/search.rs`、`src/render.rs`
- 负责预览 / 集成 / 测试的人：看 `src/build.rs`、`src/server.rs`、`tests/build.rs`

## 17. 文档维护规则

从现在开始，`README.md` 视为项目的一部分。

后续只要发生下面这些变化，就必须同步更新这份文档：

- 新增命令
- 修改配置字段
- 新增或删除重要模块
- 修改 front matter 规则
- 修改搜索行为
- 修改目录结构
- 修改页面输出规则
- 改变推荐使用方式

简单说就是：

**代码怎么变，这份用户指导就怎么跟着变。**
