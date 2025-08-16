# 文本提取去重工具 (LuniRelust CLI)

基于 Tauri + Vue + TypeScript 开发的桌面应用程序，用于从文本中提取特定信息并自动去重。

## 特性

- 🎯 **智能提取**：自动识别邮箱、URL、手机号、身份证号等
- 🔄 **自动去重**：提取后自动去重并排序
- 🎨 **可视化标记**：不同颜色标记处理状态
- 🖱️ **交互操作**：点击查看重复项关系
- 📊 **统计信息**：实时显示处理统计
- 💾 **结果导出**：一键导出处理结果

## 界面预览


```
┌──────────────────────────────────────────────────────────────┐
│ ┌───────────────────────┐          ┌─────────────────────┐   │
│ │                       │          │                     │   │
│ │     输入文本          │          │    处理结果         │   │
│ │   (支持行状态标记)     │ ┌──────┐ │   (去重后输出)      │   │
│ │                       │ │转换  │ │                     │   │
│ │                       │ └──────┘ │                     │   │
│ │                       │ ┌──────┐ │                     │   │
│ │                       │ │导出  │ │                     │   │
│ │                       │ └──────┘ │                     │   │
│ │                       │ ┌──────┐ │                     │   │
│ │                       │ │重置  │ │                     │   │
│ │                       │ └──────┘ │                     │   │
│ └───────────────────────┘          └─────────────────────┘   │
└──────────────────────────────────────────────────────────────┘
```

## 技术栈

- **前端**：Vue 3 + TypeScript + Vite
- **后端**：Rust + Tauri 2.0
- **UI**：现代化响应式设计
- **数据处理**：正则表达式 + 哈希去重

## 开发环境

### 推荐 IDE 配置

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### 依赖要求

- Node.js (推荐 v18+)
- Rust (推荐最新稳定版)
- 系统依赖请参考 [Tauri 官方文档](https://tauri.app/start/prerequisites/)

## 快速开始

```bash
# 克隆项目
git clone <repository-url>
cd lunirelust-cli

# 安装依赖
npm install

# 开发模式运行
npm run tauri dev

# 构建生产版本
npm run tauri build
```

## 使用说明

详细使用说明请查看 [README_USAGE.md](./README_USAGE.md)

## 项目结构

```
lunirelust-cli/
├── src/                  # Vue 前端源码
│   ├── App.vue          # 主应用组件
│   ├── main.ts          # 入口文件
│   └── assets/          # 静态资源
├── src-tauri/           # Tauri 后端源码
│   ├── src/
│   │   ├── lib.rs       # 核心处理逻辑
│   │   └── main.rs      # 入口文件
│   ├── Cargo.toml       # Rust 依赖配置
│   └── tauri.conf.json  # Tauri 配置
├── sample/              # 示例数据
└── xdoc/                # 设计文档
```

## 功能实现

### 后端 (Rust)

- 📝 文本解析和模式匹配
- 🔍 多种数据类型提取（邮箱、URL、手机号等）
- 🗂️ 数据去重和排序算法
- 🔄 状态管理和交互处理

### 前端 (Vue)

- 🎨 现代化 UI 设计
- 📊 实时状态可视化
- 🖱️ 交互式行选择
- 📈 统计信息面板
- 💾 文件导出功能

## 开发计划

- [x] 基础架构搭建
- [x] 核心提取算法实现
- [x] UI 界面设计和实现
- [x] 状态标记和交互功能
- [x] 统计信息和导出功能
- [ ] 自定义提取规则
- [ ] 批量文件处理
- [ ] 设置和配置项
- [ ] 多语言支持

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License
