# 🦀 Rust Tips - Rust 学习笔记

这是一个为 Rust 初学者准备的学习笔记集合。每个笔记包含概念说明、代码示例、学习建议和练习题。

## ✨ 功能特性

- 📚 按主题分类的学习笔记
- 💡 每个主题包含概念说明、代码示例和学习建议
- ⭐ 难度等级标注（⭐ 入门 → ⭐⭐⭐⭐ 专家）
- 📝 配套练习题帮助巩固知识
- 🔗 参考资料链接

## 📖 当前包含的笔记

### 基础语法 (Basics)

| 笔记 | 难度 | 描述 |
|------|------|------|
| 变量与类型 | ⭐ | let、mut、const、类型系统 |
| 函数 | ⭐ | fn 定义、参数、返回值、表达式 |
| 流程控制 | ⭐ | if、loop、while、for、match |
| 所有权 | ⭐⭐ | 所有权、借用、引用、生命周期 |

## 🚀 快速开始

### 安装 Rust

如果你还没有安装 Rust，请访问 [https://rustup.rs/](https://rustup.rs/) 安装。

### 克隆并运行

```bash
# 克隆仓库
git clone https://github.com/LoveMiku233/rust_tips.git
cd rust_tips

# 运行示例查看所有笔记
cargo run --example show_notes

# 运行测试
cargo test
```

## 📚 使用方法

### 作为库使用

```rust
use rust_tips::topics::basics;

fn main() {
    // 获取所有基础笔记
    let notes = basics::all_notes();
    
    for note in notes {
        println!("📖 {} - {}", note.meta.title, note.difficulty_stars());
        println!("概要: {}", note.content.summary);
    }
    
    // 获取单个笔记
    let ownership = basics::ownership::note();
    println!("所有权笔记代码示例:\n{}", ownership.content.code);
}
```

### 笔记结构

每个笔记包含以下内容：

```rust
Note {
    meta: NoteMeta {
        id: "笔记ID",
        title: "标题",
        category: Category::Basics,
        difficulty: Difficulty::Beginner,
        tags: &["标签1", "标签2"],
        updated: "2024-02",
    },
    content: NoteContent {
        summary: "核心概念总结",
        code: "代码示例",
        tips: "学习建议",
        exercises: Some("练习题"),
        refs: &["参考链接"],
    },
    config: RenderConfig::default(),
}
```

## 📁 项目结构

```
rust_tips/
├── Cargo.toml              # 项目配置
├── README.md               # 本文件
├── src/
│   ├── lib.rs              # 库入口
│   ├── core/               # 核心类型定义
│   │   ├── mod.rs
│   │   └── note.rs         # Note 结构体
│   └── topics/             # 学习笔记（按主题分类）
│       ├── mod.rs
│       └── basics/         # 基础语法
│           ├── mod.rs
│           ├── variables_types.rs
│           ├── functions.rs
│           ├── flow_control.rs
│           └── ownership.rs
├── examples/
│   └── show_notes.rs       # 展示笔记示例
└── tests/
    └── basics_tests.rs     # 基础模块测试
```

## 🎯 学习路线建议

作为 Rust 初学者，建议按以下顺序学习：

1. **变量与类型** - 了解 Rust 的基本语法
2. **函数** - 掌握函数定义和调用
3. **流程控制** - 学习条件判断和循环
4. **所有权** ⭐ - Rust 最核心的概念！

## 🛠️ 开发命令

```bash
# 编译项目
cargo build

# 运行测试
cargo test

# 运行特定测试
cargo test test_ownership_note

# 查看文档
cargo doc --open

# 运行示例
cargo run --example show_notes
```

## 📝 贡献指南

欢迎贡献新的学习笔记！请按照以下格式添加：

1. 在 `src/topics/` 对应分类下创建新文件
2. 实现 `pub fn note() -> Note` 函数
3. 在 `mod.rs` 中导出模块
4. 添加对应的测试
5. 更新 README.md

## 📚 推荐学习资源

- [The Rust Programming Language（官方书籍）](https://doc.rust-lang.org/book/)
- [Rust 语言圣经](https://course.rs/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings（练习项目）](https://github.com/rust-lang/rustlings)

## 📄 许可证

MIT License

---

🦀 Happy Learning Rust! 祝你学习愉快！
