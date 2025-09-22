# Bodhi Migration

一个强大的跨平台数据库迁移工具，结合了现代前端技术和高性能后端处理能力，为数据库迁移提供直观、高效的解决方案。

## 项目概述

Bodhi Migration 是一款基于 Tauri 和 SvelteKit 构建的跨平台桌面应用程序，专门用于简化和管理不同数据库系统之间的数据迁移任务。该工具支持 SQLite、MySQL 和 Redis 等主流数据库，提供友好的用户界面和强大的后端处理能力。

## 技术栈

### 前端
- **框架**: [Svelte 5](https://svelte.dev/) + [SvelteKit](https://kit.svelte.dev/)
- **语言**: [TypeScript](https://www.typescriptlang.org/)
- **构建工具**: [Vite](https://vitejs.dev/)
- **状态管理**: [@tanstack/svelte-query](https://tanstack.com/query/latest/docs/svelte/overview)
- **图表库**: [Chart.js](https://www.chartjs.org/) + [svelte-chartjs](https://github.com/SauravKanchan/svelte-chartjs)
- **Tauri API**: [@tauri-apps/api](https://github.com/tauri-apps/tauri)

### 后端
- **运行时**: [Rust](https://www.rust-lang.org/)
- **框架**: [Tauri 2](https://tauri.app/)
- **数据库驱动**: 
  - SQLite: [rusqlite](https://github.com/rusqlite/rusqlite)
  - MySQL: [mysql_async](https://github.com/blackbeam/mysql_async)
  - Redis: [redis](https://github.com/redis-rs/redis-rs)
- **异步支持**: [tokio](https://tokio.rs/)
- **序列化**: [serde](https://serde.rs/)

## 快速开始

### 前置要求

确保您的系统已安装以下软件:
- [Node.js](https://nodejs.org/) (推荐 v18+)
- [pnpm](https://pnpm.io/) (包管理器)
- [Rust](https://www.rust-lang.org/tools/install) (用于构建 Tauri 应用)
- [Git](https://git-scm.com/)

### 安装步骤

1. **克隆仓库**
```bash
git clone https://github.com/yourusername/bodhi_migration.git
cd bodhi_migration
```

2. **安装依赖**
```bash
pnpm install
```

3. **启动开发服务器**
```bash
pnpm tauri dev
```

这将启动开发服务器并打开 Bodhi Migration 应用程序。

## 开发指南

### 项目结构

```
bodhi_migration/
├── src/                 # 前端源代码
│   ├── app.html         # HTML 入口文件
│   ├── components/      # Svelte 组件
│   ├── routes/          # SvelteKit 路由
│   ├── stores/          # 状态管理
│   └── types/           # TypeScript 类型定义
├── src-tauri/           # Tauri 后端代码
│   ├── src/             # Rust 源代码
│   │   ├── database/    # 数据库操作模块
│   │   ├── migration/   # 数据迁移逻辑
│   │   └── models/      # 数据模型
│   ├── Cargo.toml       # Rust 依赖配置
│   └── tauri.conf.json  # Tauri 应用配置
├── static/              # 静态资源
├── package.json         # 前端依赖配置
└── tsconfig.json        # TypeScript 配置
```

### 主要组件

- **DatabaseConfigForm**: 数据库连接配置表单
- **TaskRunner**: 迁移任务执行器
- **MainDashboard**: 主控制面板
- **TaskCreationForm**: 迁移任务创建表单

### 命令参考

```bash
# 开发模式启动
pnpm tauri dev

# 构建前端
pnpm build

# 类型检查
pnpm check

# 持续类型检查
pnpm check:watch

# Tauri 命令
pnpm tauri [command]
```

## 功能特性

### 数据库连接管理
- 支持 SQLite、MySQL 和 Redis 数据库连接配置
- 连接测试和验证功能
- 连接信息安全存储

### 迁移任务管理
- 创建、编辑和删除迁移任务
- 配置源数据库和目标数据库
- 设置迁移规则和映射关系

### 迁移执行
- 实时迁移进度监控
- 迁移状态和结果展示
- 错误处理和日志记录

### 数据可视化
- 迁移统计图表
- 性能监控数据展示

## 构建和打包

### 为当前平台构建

```bash
pnpm tauri build
```

构建产物将位于 `src-tauri/target/release/bundle/` 目录中。

### 为特定平台构建

```bash
# Windows
pnpm tauri build --target x86_64-pc-windows-msvc

# macOS
pnpm tauri build --target x86_64-apple-darwin
# 或
pnpm tauri build --target aarch64-apple-darwin

# Linux
pnpm tauri build --target x86_64-unknown-linux-gnu
```

## 推荐开发环境

[VS Code](https://code.visualstudio.com/) + 以下扩展:
- [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [TypeScript](https://marketplace.visualstudio.com/items?itemName=ms-vscode.vscode-typescript-next)

## 贡献指南

1. Fork 项目仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 许可证

本项目使用 [MIT 许可证](LICENSE)。

## 致谢

感谢所有为本项目做出贡献的开发者和支持者。

## 联系方式

如有问题或建议，请联系项目维护者 Stanley Guo (stanleyguo0207@163.com)
