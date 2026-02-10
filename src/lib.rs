//! # Rust Tips - Rust 学习笔记库
//!
//! 这是一个为 Rust 初学者准备的学习笔记集合。
//!
//! ## 功能特性
//!
//! - 📚 按主题分类的学习笔记
//! - 💡 每个主题包含概念说明、代码示例和学习建议
//! - ⭐ 难度等级标注
//!
//! ## 使用示例
//!
//! ```rust
//! use rust_tips::topics::basics;
//!
//! // 获取流程控制笔记
//! let note = basics::flow_control::note();
//! println!("标题: {}", note.meta.title);
//! println!("难度: {}", note.difficulty_stars());
//! ```

pub mod core;
pub mod topics;

// 重新导出常用类型，方便使用
pub use core::{
    Note, NoteMeta, NoteContent, Category, Difficulty,
    RenderConfig, CodeTheme
};
