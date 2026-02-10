//! 基础语法笔记模块

pub mod flow_control;
pub mod variables_types;
pub mod functions;
pub mod ownership;

/// 获取所有基础笔记
pub fn all_notes() -> Vec<crate::core::Note> {
    vec![
        variables_types::note(),
        functions::note(),
        flow_control::note(),
        ownership::note(),
    ]
}