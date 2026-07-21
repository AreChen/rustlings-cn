# Rustlings 中文站与站内导航设计

## 目标

让 `rustlings-cn` 的 GitHub Pages 成为可独立使用的简体中文 Rustlings 网站，并在 GitHub 仓库说明和 README 中提供稳定入口。

## 范围

1. 将网站 canonical base URL 设置为 `https://arechen.github.io/rustlings-cn`，使 Zola 的内部链接、导航、目录锚点和资源 URL 都指向本站。
2. 翻译网站配置、导航、页脚、首页、安装、使用、社区练习和 404 页面中的用户可见英文。
3. 保留 Rust 官方文档、Rust 官网、GitHub 讨论区和其他社区项目等外部资源链接；将仓库自身的 Repository、Changelog、License 入口指向 `AreChen/rustlings-cn`。
4. 将 GitHub 仓库的 description 与 homepage 设置为中文说明和 Pages 地址，并在中文 README 顶部提供在线阅读入口。

## 实现设计

- Zola 负责站内 URL 生成，唯一 canonical base URL 位于 `website/config.toml`。
- 内容页继续使用 Zola 的 `@/...` 链接宏，模板继续使用 `get_url`，不手写带域名的站内链接。
- 导航和页脚标签在配置中翻译，页面内容在对应 Markdown 中翻译；命令、路径、API、代码块和外部 URL 保持可复制性。
- Website 的部署仍由现有 GitHub Actions workflow 负责，修改后通过本地 Zola 构建、HTML 官方域名扫描、链接检查和远程 Pages workflow 验证。

## 成功标准

- 网站入口返回 HTTP 200，页面语言声明为 `zh-CN`。
- 主页、Setup、Usage、Community Exercises、404 和所有导航标签均为中文。
- 构建产物中不再出现 `https://rustlings.rust-lang.org` 作为本站导航或 canonical URL。
- GitHub 仓库 homepage 为 `https://arechen.github.io/rustlings-cn/`，中文 README 可直接看到该链接。
- Website workflow 的 lint、构建、npm audit 和 Pages 部署全部通过。
