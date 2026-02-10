use crate::core::{Note, NoteMeta, NoteContent, Category, Difficulty, RenderConfig, CodeTheme};


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
            summary: "Rust流程控制...",
            code: "...",  // 你的 loop_example 代码可以放在这里
            tips: "建议...",
            exercises: None,
            refs: &[],
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