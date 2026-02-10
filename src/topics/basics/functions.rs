use crate::core::{Note, NoteMeta, NoteContent, Category, Difficulty, RenderConfig};

/// 获取函数笔记
pub fn note() -> Note {
    Note {
        meta: NoteMeta {
            id: "functions",
            title: "函数",
            category: Category::Basics,
            difficulty: Difficulty::Beginner,
            tags: &["函数", "fn", "返回值", "参数"],
            updated: "2024-02",
        },
        content: NoteContent {
            summary: r#"Rust 函数的核心概念：
1. **fn** - 函数声明关键字
2. **参数** - 必须声明类型
3. **返回值** - 使用 -> 指定类型
4. **表达式 vs 语句** - 表达式有返回值，语句没有
5. **提前返回** - 使用 return 关键字"#,
            code: r#"// 1. 基本函数定义
fn say_hello() {
    println!("Hello!");
}

// 2. 带参数的函数（必须声明类型）
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// 3. 带返回值的函数
fn add(a: i32, b: i32) -> i32 {
    a + b  // 注意：没有分号，这是表达式
}

// 4. 多个返回值（使用元组）
fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

// 5. 提前返回
fn abs(x: i32) -> i32 {
    if x < 0 {
        return -x;  // 提前返回
    }
    x  // 最后一个表达式作为返回值
}

// 6. 表达式 vs 语句
fn expression_example() -> i32 {
    let y = {
        let x = 3;
        x + 1  // 表达式，有返回值
    };
    y  // 返回 4
}

// 7. 函数作为参数
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

// 使用示例
fn main() {
    say_hello();
    greet("Rust");
    let sum = add(5, 3);
    let (b, a) = swap(1, 2);
    let result = apply(|x| x * 2, 5);  // 使用闭包
}"#,
            tips: r#"💡 学习建议：
- 函数名使用 snake_case 命名风格
- 参数类型必须显式声明，不能省略
- 最后一个表达式会自动作为返回值（不加分号）
- 加了分号就变成语句，返回 () 空元组
- Rust 函数可以在使用后再定义（不像 C 语言）"#,
            exercises: Some(r#"练习：
1. 编写一个计算阶乘的递归函数
2. 编写一个函数，接收两个数字，返回较大的那个
3. 编写一个函数，接收一个闭包并执行它"#),
            refs: &[
                "https://doc.rust-lang.org/book/ch03-03-how-functions-work.html",
                "https://course.rs/basic/base-type/function.html",
            ],
        },
        config: RenderConfig::default(),
    }
}
