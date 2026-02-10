use crate::core::{Note, NoteMeta, NoteContent, Category, Difficulty, RenderConfig};

/// 获取所有权笔记
pub fn note() -> Note {
    Note {
        meta: NoteMeta {
            id: "ownership",
            title: "所有权",
            category: Category::Basics,
            difficulty: Difficulty::Intermediate,
            tags: &["所有权", "借用", "引用", "生命周期"],
            updated: "2024-02",
        },
        content: NoteContent {
            summary: r#"所有权是 Rust 最核心的概念，三大规则：
1. 每个值都有一个**所有者**（owner）
2. 同一时刻只能有一个所有者
3. 当所有者离开作用域，值被丢弃

相关概念：
- **Move** - 所有权转移
- **Clone** - 深拷贝
- **Copy** - 栈上数据的自动复制
- **借用** - 引用数据而不获取所有权"#,
            code: r#"// 1. 所有权转移（Move）
let s1 = String::from("hello");
let s2 = s1;  // s1 的所有权转移给 s2
// println!("{}", s1);  // 错误！s1 已经无效

// 2. 克隆（深拷贝）
let s1 = String::from("hello");
let s2 = s1.clone();  // 深拷贝
println!("s1 = {}, s2 = {}", s1, s2);  // OK

// 3. Copy trait（栈上数据自动复制）
let x = 5;
let y = x;  // i32 实现了 Copy，自动复制
println!("x = {}, y = {}", x, y);  // OK

// 4. 函数与所有权
fn takes_ownership(s: String) {
    println!("{}", s);
}  // s 在这里被丢弃

fn makes_copy(x: i32) {
    println!("{}", x);
}  // x 是 Copy 类型，不影响原值

let s = String::from("hello");
takes_ownership(s);  // s 的所有权转移到函数
// println!("{}", s);  // 错误！s 已经无效

let x = 5;
makes_copy(x);  // x 被复制
println!("{}", x);  // OK

// 5. 引用与借用
fn calculate_length(s: &String) -> usize {
    s.len()
}  // s 是引用，不会丢弃原值

let s = String::from("hello");
let len = calculate_length(&s);  // 借用 s
println!("长度: {}", len);  // s 仍然有效

// 6. 可变引用
fn change(s: &mut String) {
    s.push_str(", world");
}

let mut s = String::from("hello");
change(&mut s);
println!("{}", s);  // "hello, world"

// 7. 引用规则
// - 同一时刻，只能有一个可变引用，或多个不可变引用
// - 引用必须始终有效（不能有悬垂引用）"#,
            tips: r#"💡 学习建议：
- 所有权是 Rust 的核心，一定要理解透彻
- String 在堆上，有所有权；&str 是引用
- 基本类型（i32, bool, char 等）实现了 Copy
- 函数参数传递遵循所有权规则
- 优先使用引用，避免不必要的 clone
- 可变引用只能有一个，防止数据竞争"#,
            exercises: Some(r#"练习：
1. 编写一个函数，接收 String 并返回其所有权
2. 修改一个使用 clone 的代码，改为使用引用
3. 解释为什么以下代码无法编译：
   let mut s = String::from("hello");
   let r1 = &mut s;
   let r2 = &mut s;
   println!("{}, {}", r1, r2);"#),
            refs: &[
                "https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html",
                "https://course.rs/basic/ownership/ownership.html",
            ],
        },
        config: RenderConfig::default(),
    }
}
