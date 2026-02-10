//! 示例：展示所有笔记
//!
//! 运行方式：cargo run --example show_notes

use rust_tips::topics::basics;

fn main() {
    println!("=== Rust Tips - Rust 学习笔记 ===\n");

    let notes = basics::all_notes();

    for note in notes {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📖 {} {}", note.meta.title, note.difficulty_stars());
        println!("📁 分类: {} | 🏷️  标签: {}", note.meta.category, note.meta.tags.join(", "));
        println!("📅 更新: {}", note.meta.updated);
        println!();
        println!("📝 概要:");
        println!("{}", note.content.summary);
        println!();
        println!("💻 代码示例:");
        println!("```rust");
        println!("{}", note.content.code);
        println!("```");
        println!();
        println!("{}", note.content.tips);

        if let Some(exercises) = note.content.exercises {
            println!();
            println!("📝 {}", exercises);
        }

        if !note.content.refs.is_empty() {
            println!();
            println!("🔗 参考资料:");
            for r in note.content.refs {
                println!("  - {}", r);
            }
        }
        println!();
    }
}
