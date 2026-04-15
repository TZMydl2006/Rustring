# MiniZensical 用户指导

## 1. 这个项目是做什么的

`MiniZensical` 是一个用 Rust 写的简化版静态站点生成器，灵感来自 zensical。

它做的事情很简单：

1. 读取根目录下的 `zensical.toml`
2. 扫描 `docs/` 目录里的 Markdown 和静态资源
3. 把 Markdown 转成 HTML
4. 生成左侧导航、页内目录、上一页/下一页链接
5. 把结果输出到 `site/`

如果你把它想成一句话，就是：

`zensical.toml + docs/ -> site/`

## 2. 第一次接触时，先记住这几个目录

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
  Rust 项目的配置文件，定义项目名、版本和依赖库。

- `Cargo.lock`
  锁定依赖版本，保证不同机器上编译结果更稳定。

- `README.md`
  就是这份说明文档。之后项目结构或使用方式变化时，这份文档也应该一起更新。

- `zensical.toml`
  项目的用户配置文件。它决定站点名称、输入目录、输出目录、URL 风格、导航结构等。

- `docs/`
  文档源文件目录。你们主要写的 Markdown 页面、图片、额外静态资源都放在这里。

- `site/`
  构建产物目录。每次执行 `build` 都会重新生成。不要手动改这里面的 HTML/CSS，因为下次构建会覆盖。

- `src/`
  Rust 源码目录。项目逻辑都在这里。

- `tests/`
  集成测试目录，用来验证这个项目的核心能力是不是正常。

- `target/`
  Cargo 编译缓存目录。它是工具自动生成的，不是你们日常阅读代码的重点。

## 3. `src/` 里每个文件是干什么的

### `src/main.rs`

命令行入口。

它负责：

- 解析命令，例如 `minizensical build` 和 `minizensical serve`
- 接收 `--config zensical.toml`
- 调用 `Config::load(...)`
- 根据子命令进入构建流程或本地预览流程

你可以把它理解成“程序启动后，第一个被执行的地方”。

### `src/lib.rs`

库入口。

它负责把主要模块导出出来，比如：

- `build`
- `config`
- `markdown`
- `nav`
- `page`
- `render`
- `scanner`
- `server`
- `error`

如果以后你们想把这个项目当成 Rust 库复用，这里就是统一出口。

### `src/error.rs`

统一错误定义。

它的作用是让项目在失败时能给出更清楚的信息，比如：

- 读文件失败
- 扫描目录失败
- `toml` 解析失败
- 用户配置不合法
- 模板渲染失败

这样出现问题时，不会只看到一串难懂的 panic。

### `src/config.rs`

读取和校验 `zensical.toml` 的模块。

它负责：

- 把 `zensical.toml` 解析成 Rust 结构体
- 提供默认值
- 校验 `docs_dir` 和 `site_dir`
- 校验 `nav` 的写法是否正确
- 生成 `docs_dir()` 和 `site_dir()` 这样的辅助方法

如果以后你们要给配置文件增加新字段，通常从这里开始改。

### `src/scanner.rs`

扫描 `docs/` 目录的模块。

它负责：

- 遍历 `docs/` 下的所有文件
- 判断哪些是 Markdown，哪些是普通静态资源
- 把文件路径整理成统一格式

它输出的是“原始输入文件列表”，供后面的页面生成和资源复制使用。

### `src/markdown.rs`

Markdown 渲染模块。

它负责：

- 把 Markdown 转成 HTML
- 提取标题
- 给标题生成锚点 id
- 生成页内目录 TOC

它是“Markdown 文本 -> 页面内容数据”的核心一步。

### `src/page.rs`

页面模型模块。

它负责定义一个页面到底包含什么，比如：

- 源文件路径
- 输出文件路径
- 页面标题
- HTML 正文
- TOC
- 规范化 URL

它还负责实现输出路径规则，比如：

- `index.md -> index.html`
- `guide/setup.md -> guide/setup/index.html`，当 `use_directory_urls = true`
- `guide/setup.md -> guide/setup.html`，当 `use_directory_urls = false`

### `src/nav.rs`

导航模块。

它负责两种导航方式：

- 自动导航：如果 `zensical.toml` 没写 `nav`，就按目录结构自动生成
- 显式导航：如果写了 `nav`，就按配置顺序和标题生成

它还负责：

- 计算当前页面对应的 active 导航项
- 生成上一页和下一页链接
- 计算页面之间的相对链接

如果你们未来要加入“折叠导航”“面包屑”“更复杂的 section 逻辑”，这里会是重点修改点。

### `src/render.rs`

渲染 HTML 的模块。

它负责：

- 使用内置模板 `main.html`
- 把页面内容、导航、TOC、前后页链接注入模板
- 提供内置样式 `minizensical.css`

如果你们之后想加入自己的页面风格、学校课程展示风格、彩蛋页面、主题切换，这里是最直接的入口。

### `src/build.rs`

总调度模块。

它是整个项目的“总装配线”。

它负责按顺序做这些事：

1. 清空旧的 `site/`
2. 扫描 `docs/`
3. 生成所有页面
4. 构建导航
5. 写入内置 CSS
6. 渲染 HTML 页面
7. 复制静态资源

如果你想快速理解整个项目从输入到输出的流程，优先看这个文件。

### `src/server.rs`

本地预览服务器模块。

它负责：

- 在启动服务前先执行一次 `build`
- 监听本地地址，例如 `127.0.0.1:3000`
- 把 `site/` 中生成好的 HTML、CSS、图片等文件通过 HTTP 提供出去
- 处理目录 URL、普通 `.html` 路径和静态资源请求

当前它会先构建一次，再在后台轮询 `docs/` 和 `zensical.toml` 的变化；检测到变更并成功重建后，会让打开中的预览页面自动刷新。

## 4. `tests/` 里是什么

### `tests/build.rs`

这是集成测试。

它现在覆盖了这些核心场景：

- 最小站点能否正常生成
- 目录 URL 模式是否正确
- 静态资源是否被复制
- 显式导航是否覆盖标题和顺序
- 缺失导航页面时是否报错
- `use_directory_urls = false` 是否生成 `.html` 文件

如果你们改了核心逻辑，最好先跑一遍测试，确保没有把现有能力改坏。

另外，`src/server.rs` 里也有针对预览服务器路径解析的单元测试。

## 5. `docs/` 和 `site/` 的关系

### `docs/`

这是输入。

例如：

- `docs/index.md`
- `docs/guide/index.md`
- `docs/guide/setup.md`
- `docs/guide/resources.md`
- `docs/assets/logo.png`

### `site/`

这是输出。

构建后会变成类似：

- `site/index.html`
- `site/guide/index.html`
- `site/guide/setup/index.html`
- `site/guide/resources/index.html`
- `site/assets/logo.png`

简而言之：

- 写内容，看 `docs/`
- 看结果，看 `site/`
- 改逻辑，看 `src/`

## 6. 这个项目现在支持什么

当前 MVP 已经支持：

- 读取 `zensical.toml`
- 扫描 `docs/`
- Markdown 转 HTML
- 自动提取 H1 作为标题
- 生成页内目录
- 自动导航
- 显式导航
- 上一页 / 下一页链接
- 复制静态资源
- 输出可直接打开的 HTML 页面
- 本地预览服务器

## 7. 这个项目现在还不支持什么

为了让第一阶段简单稳定，目前故意没有做：

- `mkdocs.yml` 兼容
- Python 兼容层
- front matter
- 搜索
- watch / 热更新
- 插件系统
- 多主题 / 主题覆盖

如果你们第二阶段要扩展，这些都是可选方向。

## 8. 如何使用这个项目

### 步骤 1：进入项目目录

```bash
cd /Users/wangyilin/Downloads/西交/Rust程序设计/RustProject/minizensical
```

### 步骤 2：准备配置文件

项目根目录已经有一个示例 `zensical.toml`。

当前示例：

```toml
[project]
site_name = "MiniZensical Demo"
docs_dir = "docs"
site_dir = "site"
use_directory_urls = true

nav = [
  { title = "Home", path = "index.md" },
  { title = "Guide", children = [
    { title = "Overview", path = "guide/index.md" },
    { title = "Setup", path = "guide/setup.md" },
    { title = "Resources", path = "guide/resources.md" },
  ] }
]
```

### 步骤 3：在 `docs/` 中写 Markdown

比如：

```text
docs/
├── index.md
└── guide/
    ├── index.md
    ├── setup.md
    └── resources.md
```

### 步骤 4：执行构建

```bash
cargo run -- build
```

如果你要显式指定配置文件：

```bash
cargo run -- build --config zensical.toml
```

### 步骤 5：本地预览

```bash
cargo run -- serve
```

或者自定义地址：

```bash
cargo run -- serve --addr 127.0.0.1:4000
```

默认预览地址是：

```text
http://127.0.0.1:3000
```

注意：

- `serve` 会先自动执行一次构建
- 它会自动监听 `docs/` 和 `zensical.toml` 的变化并重新构建
- 成功重建后，预览页面会自动刷新

### 步骤 6：查看生成结果

构建完成后，打开：

```text
site/index.html
```

直接用浏览器打开这个文件即可。

## 9. 配置文件怎么写

配置文件路径默认是项目根目录下的 `zensical.toml`。

### `[project]`

这是主配置块。

### `site_name`

站点名称。会显示在页面标题和侧边栏品牌位置。

示例：

```toml
site_name = "Rust Course Docs"
```

### `docs_dir`

文档源目录，默认是 `docs`。

示例：

```toml
docs_dir = "docs"
```

### `site_dir`

生成目录，默认是 `site`。

示例：

```toml
site_dir = "site"
```

### `use_directory_urls`

决定输出链接风格。

如果是 `true`：

- `guide/setup.md -> site/guide/setup/index.html`

如果是 `false`：

- `guide/setup.md -> site/guide/setup.html`

### `site_url`

可选。

如果填写，会生成页面 canonical URL。

示例：

```toml
site_url = "https://example.com"
```

### `nav`

可选。

如果不写，就按目录结构自动生成导航。

如果写了，就按你定义的标题和顺序生成导航。

叶子节点写法：

```toml
{ title = "Home", path = "index.md" }
```

分组节点写法：

```toml
{ title = "Guide", children = [
  { title = "Overview", path = "guide/index.md" },
  { title = "Resources", path = "guide/resources.md" }
] }
```

注意：

- 一个导航项不能同时写 `path` 和 `children`
- 一个导航项必须至少写其中一个
- `path` 是相对于 `docs/` 的路径

## 10. 日常最常见的操作

### 新增一页文档

1. 在 `docs/` 下新建一个 `.md` 文件
2. 如果你使用显式导航，再去 `zensical.toml` 的 `nav` 里加上它
3. 重新执行 `cargo run -- build`

### 修改站点标题

改 `zensical.toml`：

```toml
site_name = "新的标题"
```

### 添加图片或其他资源

把资源直接放进 `docs/` 里，例如：

```text
docs/assets/logo.png
```

构建后会复制到：

```text
site/assets/logo.png
```

Markdown 里这样引用图片：

```md
![Logo](assets/logo.png)
```

如果当前页面在 `docs/guide/` 下面，就写：

```md
![Logo](../assets/logo.png)
```

如果你只想做下载链接：

```md
[下载 Logo](assets/logo.png)
```

你们当前仓库里已经有一个实际示例文件：

```text
docs/assets/交大校徽-蓝色.png
```

可以直接参考：

- 首页 `docs/index.md`
- 资源页 `docs/guide/resources.md`

### 不想手写导航

删除 `zensical.toml` 里的 `nav`，系统会自动按目录结构生成。

### 想本地预览站点

运行：

```bash
cargo run -- serve
```

然后在浏览器打开：

```text
http://127.0.0.1:3000
```

当 `serve` 正在运行时：

- 修改 `docs/` 下的 Markdown 会自动重建
- 修改 `docs/` 下的图片或其他静态资源也会自动重建
- 修改 `zensical.toml` 也会自动重建
- 成功重建后，浏览器页面会自动刷新

### 想让链接变成 `.html`

把：

```toml
use_directory_urls = true
```

改成：

```toml
use_directory_urls = false
```

## 11. 如果你要读代码，建议从哪里开始

如果你是第一次接触这个项目，我建议阅读顺序如下：

1. 先看 `src/main.rs`
   了解命令怎么进来

2. 再看 `src/build.rs`
   了解整体构建流水线

3. 再看 `src/config.rs`
   了解配置是怎么进入系统的

4. 再看 `src/scanner.rs`
   了解输入文件是怎么被识别的

5. 再看 `src/page.rs` + `src/markdown.rs`
   了解 Markdown 页面是怎么建模和转换的

6. 再看 `src/nav.rs`
   了解导航和前后页逻辑

7. 再看 `src/render.rs`
   了解页面最终是怎么被拼装成 HTML 的

8. 如果你关心预览功能，再看 `src/server.rs`
   了解本地静态服务器如何工作

## 12. 如果你们之后要扩展功能，应该改哪里

### 想加搜索

优先看：

- `src/build.rs`
- `src/page.rs`
- `src/render.rs`

思路通常是：

- 先在构建阶段收集页面索引
- 生成 `search.json`
- 再在前端模板里加搜索框和搜索脚本

### 想加 front matter

优先看：

- `src/markdown.rs`
- `src/page.rs`
- `src/config.rs`

### 想增强本地预览服务器

优先看：

- `src/main.rs`
- `src/build.rs`
- `src/server.rs`

### 想改页面样式

优先看：

- `src/render.rs`

这里同时包含模板和内置 CSS。

### 想改导航逻辑

优先看：

- `src/nav.rs`

## 13. 常见问题

### 为什么我改了 `site/` 里的 HTML，但下次又没了？

因为 `site/` 是生成目录，每次 `build` 都会重建。应该改的是：

- 内容：`docs/`
- 样式和模板：`src/render.rs`
- 配置：`zensical.toml`

### 为什么我在导航里写了路径却报错？

因为 `nav.path` 必须指向 `docs/` 里真实存在的 Markdown 文件，并且路径必须相对 `docs/`。

### 为什么页面标题和 Markdown 第一个标题不一样？

如果你在显式导航里写了 `title`，导航标题会覆盖页面显示标题，这是当前 MVP 的设计。

### 为什么我新增文档后左侧没显示？

可能有两个原因：

- 你使用的是显式导航，但没有在 `zensical.toml` 的 `nav` 中加入新页面
- 你还没有重新执行 `cargo run -- build`

如果你使用的是 `serve`，当前版本会在成功重建后自动刷新浏览器页面。

## 14. 当前推荐的工作方式

对于你们四个人协作，我建议这样分工时阅读代码：

- 负责 CLI / 配置的人，重点看 `src/main.rs` 和 `src/config.rs`
- 负责 Markdown / 页面的人，重点看 `src/markdown.rs` 和 `src/page.rs`
- 负责导航 / UI 的人，重点看 `src/nav.rs` 和 `src/render.rs`
- 负责服务端预览的人，重点看 `src/server.rs`
- 负责整体集成 / 测试的人，重点看 `src/build.rs`、`src/server.rs` 和 `tests/build.rs`

## 15. 文档维护规则

从现在开始，这份 `README.md` 视为项目的一部分。

只要后续发生下面这些变化，就应该同步更新这份文档：

- 新增命令
- 修改配置字段
- 修改目录结构
- 修改页面输出规则
- 新增主要模块
- 改变推荐使用方式

简单说就是：

代码怎么变，这份用户指导就怎么跟着变。
