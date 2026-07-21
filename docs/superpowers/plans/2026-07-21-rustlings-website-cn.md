# Rustlings 中文站与站内导航实施计划

> **面向 Agent 型工作者：** 必需的子技能：使用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 来逐项任务实施此计划。步骤使用复选框（`- [ ]`）语法进行跟踪。

**目标：** 将 Rustlings 中文版的 GitHub Pages、站内导航和用户可见内容完整切换为中文站体验。

**架构：** 使用 Zola `base_url` 作为唯一站点 URL 来源，保留 `@/...` 内容链接和 `get_url` 模板链接，让所有站内 URL 自动带上 GitHub Pages 项目路径。网站文案集中修改 `website/config.toml`、模板和四个内容页，仓库入口同步写入 GitHub 元数据、README 和 Cargo manifest。

**技术栈：** Zola、Tera 模板、Markdown、Tailwind CLI、GitHub Actions、GitHub CLI。

## 全局约束

- Rustlings CLI 最低 Rust 版本保持 `1.88`，不要因网站文案修改依赖或练习行为。
- 保持命令、文件路径、代码标识符、API 名称和可复制代码块不翻译。
- Rust 官方文档、Rust 官网、官方 GitHub 讨论区和其他社区项目链接保持外部链接。
- 站内链接必须使用 Zola 的 `@/...` 或模板的 `get_url`，不得手写官方 Rustlings 网站域名。

---

### 任务 1: 更新网站站点身份与链接生成

**文件：**

- 修改：`website/config.toml`
- 修改：`website/templates/base.html`
- 修改：`website/templates/404.html`

**步骤：**

- [x] **步骤 1：** 将 `base_url` 改为 `https://arechen.github.io/rustlings-cn`，把导航和页脚标签翻译为中文，并把 Repository、Changelog、MIT License 指向中文 fork。
- [x] **步骤 2：** 将 HTML `lang` 改为 `zh-CN`，翻译页脚声明和 404 页面按钮文案。
- [x] **步骤 3：** 保持模板中的 `get_url` 和内容页 permalink 逻辑不变，确保它们从新的 `base_url` 生成本站 URL。

### 任务 2: 翻译网站内容

**文件：**

- 修改：`website/templates/index.html`
- 修改：`website/content/_index.md`
- 修改：`website/content/setup/index.md`
- 修改：`website/content/usage/index.md`
- 修改：`website/content/community-exercises/index.md`

**步骤：**

- [x] **步骤 1：** 翻译主页简介、快速开始和引导链接，并使用中文 fork 的安装命令。
- [x] **步骤 2：** 翻译安装页、初始化说明、编辑器和终端建议、离线文档说明及错误提示。
- [x] **步骤 3：** 翻译使用页、练习列表、Watch Mode、帮助和后续学习说明。
- [x] **步骤 4：** 翻译社区练习页，在列表中明确标出本中文 fork，同时保留其他社区项目的原始链接。

### 任务 3: 补充仓库入口信息

**文件：**

- 修改：`README.md`
- 修改：`Cargo.toml`
- 修改：GitHub 仓库 metadata（`AreChen/rustlings-cn`）

**步骤：**

- [x] **步骤 1：** 在 README 标题后的介绍区域加入中文 Website 在线地址。
- [x] **步骤 2：** 将 Cargo package 的 repository 和 description 与中文 fork 保持一致。
- [x] **步骤 3：** 设置 GitHub repository description 和 homepage，使仓库首页直接显示中文站入口。

### 任务 4: 验证并发布

**文件：**

- 验证：`website/public/` 构建产物
- 验证：`website/content/` 与 `website/templates/` 的链接
- 验证：GitHub Actions Website workflow

**步骤：**

- [x] **步骤 1：** 运行 `npm ci`、Tailwind CSS 构建和 Zola 构建。
- [x] **步骤 2：** 扫描构建产物中的官方站点域名，确认它只出现在允许的外部资源说明中，不出现在站内导航和 canonical URL 中。
- [x] **步骤 3：** 检查关键页面 HTML 的中文标题、`lang="zh-CN"` 和本站链接。
- [ ] **步骤 4：** 提交并推送修改，触发 Website workflow，确认 lint、构建、npm audit 和 Pages 部署全部通过。
