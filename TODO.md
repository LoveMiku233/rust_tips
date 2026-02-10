src/
├── lib.rs              # 只放入口和重导出
├── core/               # 核心类型（Note定义在这里）
│   ├── mod.rs          # pub mod note;
│   └── note.rs         # Note结构体 + 基础实现
├── renderer/           # 未来：不同的输出方式
│   ├── mod.rs
│   ├── console.rs      # 终端彩色输出（当前）
│   └── html.rs         # 未来：生成网页
├── storage/            # 未来：持久化
│   └── mod.rs          # 未来：JSON/数据库保存
└── topics/             # 所有笔记内容（按技术领域分）
├── mod.rs
├── basics/         # 基础语法
├── stdlib/         # 标准库
├── graphics/       # 图形库（bevy/egui等）
├── async_/         # 异步编程
└── networking/     # 网络编程