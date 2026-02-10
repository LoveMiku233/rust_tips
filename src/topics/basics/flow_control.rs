use crate::core::{Note, NoteMeta, NoteContent, Category, Difficulty, RenderConfig};


/// 获取流程控制笔记
pub fn note() -> Note {
    Note {
        meta: NoteMeta {
            id: "flow_control",
            title: "流程控制",
            category: Category::Basics,
            difficulty: Difficulty::Beginner,
            tags: &["循环", "条件", "match"],
            updated: "2024-02",
        },
        content: NoteContent {
            summary: r#"Rust 提供了多种流程控制结构：
1. **if/else** - 条件判断，可以作为表达式返回值
2. **loop** - 无限循环，可通过 break 返回值
3. **while** - 条件循环
4. **for** - 遍历迭代器，最常用且安全
5. **match** - 模式匹配，强大的分支控制"#,
            code: r#"// 1. if 表达式（可以返回值）
let number = 5;
let result = if number > 0 { "正数" } else { "非正数" };

// 2. loop 循环（可以返回值）
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // 返回 20
    }
};

// 3. while 循环
let mut n = 3;
while n != 0 {
    println!("{}!", n);
    n -= 1;
}

// 4. for 循环（推荐使用）
let arr = [10, 20, 30];
for element in arr.iter() {
    println!("值: {}", element);
}

// 使用 range
for i in (1..4).rev() {
    println!("{}!", i);
}

// 5. match 模式匹配
let x = 1;
match x {
    1 => println!("一"),
    2 => println!("二"),
    _ => println!("其他"),
}"#,
            tips: r#"💡 学习建议：
- 优先使用 for 循环遍历，比 while 更安全，不会出现索引越界
- loop 适合需要返回值的无限循环场景
- if 和 match 都是表达式，可以直接赋值给变量
- match 必须穷尽所有可能，使用 _ 作为兜底"#,
            exercises: Some(r#"练习：
1. 使用 loop 计算 1 到 100 的和
2. 使用 for 循环打印九九乘法表
3. 使用 match 实现一个简单的计算器"#),
            refs: &[
                "https://doc.rust-lang.org/book/ch03-05-control-flow.html",
                "https://course.rs/basic/flow-control.html",
            ],
        },
        config: RenderConfig::default(),
    }
}

pub fn loop_example() {
    // loop 循环
    let log_name = "LOOP";
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("[{}] The result is {}", log_name, result);

    // while 循环
    let log_name = "WHILE";
    let mut number = 3;
    while number != 0 {
        println!("[{}] {}!", log_name, number);
        number -= 1;
    }
    println!("[{}] LIFTOFF!!!", log_name);

    // 遍历数组
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("[{}] The value is: {}", log_name, a[index]);
        index += 1;
    }

    let log_name = "FOR";
    // 使用iter引用
    for element in a.iter() {
        println!("[{}] The value is: {}", log_name, element);
    }

    // range 指定一个开始数字和结束数字，不含结束
    for number in (1..4).rev() {
        println!("[{}] {}!", log_name, number);
    }
    println!("[{}] LIFTOFF!!!", log_name);
}