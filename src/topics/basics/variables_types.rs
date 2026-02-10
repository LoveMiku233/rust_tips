use crate::core::{Note, NoteMeta, NoteContent, Category, Difficulty, RenderConfig};

/// 获取变量与类型笔记
pub fn note() -> Note {
    Note {
        meta: NoteMeta {
            id: "variables_types",
            title: "变量与类型",
            category: Category::Basics,
            difficulty: Difficulty::Beginner,
            tags: &["变量", "类型", "let", "const", "mut"],
            updated: "2024-02",
        },
        content: NoteContent {
            summary: r#"Rust 是静态类型语言，变量相关概念：
1. **let** - 声明不可变变量（默认）
2. **let mut** - 声明可变变量
3. **const** - 常量，必须标注类型
4. **shadowing** - 变量遮蔽，可以改变类型
5. **类型推断** - 编译器自动推断类型"#,
            code: r#"// 1. 不可变变量（默认）
let x = 5;
// x = 6;  // 错误！不能修改不可变变量

// 2. 可变变量
let mut y = 5;
y = 6;  // OK

// 3. 常量（必须标注类型，全大写命名）
const MAX_POINTS: u32 = 100_000;

// 4. 变量遮蔽（shadowing）- 可以改变类型
let spaces = "   ";
let spaces = spaces.len();  // OK，类型从 &str 变为 usize

// 5. 基本类型
let integer: i32 = 42;       // 有符号整数
let unsigned: u32 = 42;      // 无符号整数
let float: f64 = 3.14;       // 浮点数
let boolean: bool = true;    // 布尔值
let character: char = '中';  // 字符（支持Unicode）

// 6. 复合类型
let tuple: (i32, f64, char) = (500, 6.4, '1');
let (x, y, z) = tuple;       // 解构
let first = tuple.0;         // 索引访问

let array: [i32; 5] = [1, 2, 3, 4, 5];
let first = array[0];        // 索引访问
let same = [3; 5];           // [3, 3, 3, 3, 3]"#,
            tips: r#"💡 学习建议：
- Rust 变量默认不可变，这是一个安全特性
- 使用 mut 时要谨慎，考虑是否真的需要修改
- shadowing 不同于 mut，它创建了新变量
- 整数默认 i32，浮点默认 f64
- 数字可以用下划线分隔提高可读性：100_000"#,
            exercises: Some(r#"练习：
1. 声明一个不可变变量，尝试修改它，观察编译错误
2. 使用 shadowing 将字符串转换为其长度
3. 创建一个包含不同类型的元组并解构它"#),
            refs: &[
                "https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html",
                "https://doc.rust-lang.org/book/ch03-02-data-types.html",
            ],
        },
        config: RenderConfig::default(),
    }
}
