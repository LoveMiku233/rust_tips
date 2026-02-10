//! 集成测试 - 测试笔记模块

use rust_tips::core::{Category, Difficulty};
use rust_tips::topics::basics;

#[test]
fn test_flow_control_note() {
    let note = basics::flow_control::note();

    assert_eq!(note.meta.id, "flow_control");
    assert_eq!(note.meta.title, "流程控制");
    assert!(matches!(note.meta.category, Category::Basics));
    assert!(matches!(note.meta.difficulty, Difficulty::Beginner));
    assert!(!note.meta.tags.is_empty());
    assert!(!note.content.summary.is_empty());
    assert!(!note.content.code.is_empty());
    assert!(!note.content.tips.is_empty());
}

#[test]
fn test_variables_types_note() {
    let note = basics::variables_types::note();

    assert_eq!(note.meta.id, "variables_types");
    assert_eq!(note.meta.title, "变量与类型");
    assert!(matches!(note.meta.category, Category::Basics));
    assert!(matches!(note.meta.difficulty, Difficulty::Beginner));
    assert!(note.meta.tags.contains(&"变量"));
    assert!(note.meta.tags.contains(&"类型"));
}

#[test]
fn test_functions_note() {
    let note = basics::functions::note();

    assert_eq!(note.meta.id, "functions");
    assert_eq!(note.meta.title, "函数");
    assert!(matches!(note.meta.category, Category::Basics));
    assert!(matches!(note.meta.difficulty, Difficulty::Beginner));
}

#[test]
fn test_ownership_note() {
    let note = basics::ownership::note();

    assert_eq!(note.meta.id, "ownership");
    assert_eq!(note.meta.title, "所有权");
    assert!(matches!(note.meta.category, Category::Basics));
    // 所有权难度应该是中级
    assert!(matches!(note.meta.difficulty, Difficulty::Intermediate));
}

#[test]
fn test_all_notes_returns_correct_count() {
    let notes = basics::all_notes();
    assert_eq!(notes.len(), 4, "应该有4个基础笔记");
}

#[test]
fn test_difficulty_stars() {
    let beginner_note = basics::variables_types::note();
    assert_eq!(beginner_note.difficulty_stars(), "⭐");

    let intermediate_note = basics::ownership::note();
    assert_eq!(intermediate_note.difficulty_stars(), "⭐⭐");
}

#[test]
fn test_category_display() {
    let note = basics::flow_control::note();
    let category_str = format!("{}", note.meta.category);
    assert_eq!(category_str, "基础语法");
}

#[test]
fn test_note_has_references() {
    let note = basics::ownership::note();
    assert!(!note.content.refs.is_empty(), "所有权笔记应该有参考资料");
}

#[test]
fn test_note_has_exercises() {
    let note = basics::flow_control::note();
    assert!(note.content.exercises.is_some(), "流程控制笔记应该有练习题");
}
