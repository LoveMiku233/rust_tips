use std::fmt;

#[derive(Debug, Clone)]
pub struct Note {
    pub meta: NoteMeta,
    pub content: NoteContent,
    pub config: RenderConfig,
}


#[derive(Debug, Clone)]
pub struct NoteMeta {
    pub id: &'static str,
    pub title: &'static str,
    pub category: Category,
    pub tags: &'static [&'static str],
    pub difficulty: Difficulty,
    pub updated: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub enum Category {
    Basics,
    StdLib,
    Graphics,
    Async_,
    Networking,
    Web,
    Embedded,
    Testing,
    Tooling
}

#[derive(Debug, Clone)]
pub struct NoteContent {
    pub summary: &'static str,      // 核心概念
    pub code: &'static str,         // 代码示例
    pub tips: &'static str,         // 学习提示
    pub exercises: Option<&'static str>, // 练习题（可选）
    pub refs: &'static [&'static str],   // 参考资料链接
}

/// 渲染配置 - 控制如何显示
#[derive(Debug, Clone, Copy)]
pub struct RenderConfig {
    pub color_enabled: bool,
    pub code_theme: CodeTheme,
    pub compact_mode: bool,       // 简洁模式（只显示summary和code）
}

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Beginner,     // ⭐
    Intermediate, // ⭐⭐
    Advanced,     // ⭐⭐⭐
    Expert,       // ⭐⭐⭐⭐
}

#[derive(Debug, Clone, Copy)]
pub enum CodeTheme {
    Dark,
    Light,
    NoColor,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            color_enabled: true,
            code_theme: CodeTheme::Dark,
            compact_mode: false,
        }
    }
}

impl Default for CodeTheme {
    fn default() -> Self {
        Self::Dark
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Basics => write!(f, "基础语法"),
            Category::StdLib => write!(f, "标准库"),
            Category::Graphics => write!(f, "图形库"),
            Category::Async_ => write!(f, "异步编程"),
            Category::Networking => write!(f, "网络编程"),
            Category::Web => write!(f, "Web开发"),
            Category::Embedded => write!(f, "嵌入式"),
            Category::Testing => write!(f, "测试"),
            Category::Tooling => write!(f, "工具链"),
        }
    }
}

impl Note {
    // 构建器模式
    pub const fn builder() -> NoteBuilder {
        NoteBuilder::new()
    }

    pub fn difficulty_stars(&self) -> &'static str {
        match self.meta.difficulty {
            Difficulty::Beginner => "⭐",
            Difficulty::Intermediate => "⭐⭐",
            Difficulty::Advanced => "⭐⭐⭐",
            Difficulty::Expert => "⭐⭐⭐⭐",
        }
    }
}

pub struct NoteBuilder;

impl NoteBuilder {
    pub const fn new() -> Self {
        Self
    }
}
