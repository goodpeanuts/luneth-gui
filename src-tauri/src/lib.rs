use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineResult {
    pub line_number: usize,
    pub original_text: String,
    pub extracted_content: Option<String>,
    pub status: LineStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineStatus {
    Normal,
    Error,     // 无法提取内容
    Duplicate, // 有重复
    Selected,  // 被选中
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
    pub input_lines: Vec<LineResult>,
    pub output_lines: Vec<String>,
    pub duplicate_groups: HashMap<String, Vec<usize>>, // 提取内容 -> 行号列表
}

// 文本处理核心逻辑
fn extract_content_from_line(line: &str) -> Option<String> {
    // 尝试提取邮箱地址
    let email_re = Regex::new(r"([a-zA-Z]+)-(\d+)").unwrap();
    if let Some(mat) = email_re.find(line) {
        return Some(mat.as_str().to_owned());
    }

    // // 尝试提取URL
    // let url_re = Regex::new(r"https?://[^\s]+").unwrap();
    // if let Some(mat) = url_re.find(line) {
    //     return Some(mat.as_str().to_string());
    // }

    // // 尝试提取手机号（中国）
    // let phone_re = Regex::new(r"1[3-9]\d{9}").unwrap();
    // if let Some(mat) = phone_re.find(line) {
    //     return Some(mat.as_str().to_string());
    // }

    // // 尝试提取身份证号（简单格式）
    // let id_re = Regex::new(r"\d{17}[\dXx]|\d{15}").unwrap();
    // if let Some(mat) = id_re.find(line) {
    //     return Some(mat.as_str().to_string());
    // }

    // // 如果都没有匹配，尝试提取数字和字母组合（至少6位）
    // let general_re = Regex::new(r"[A-Za-z0-9]{6,}").unwrap();
    // if let Some(mat) = general_re.find(line) {
    //     return Some(mat.as_str().to_string());
    // }

    None
}

fn process_text_lines(input: &str) -> ProcessResult {
    let lines: Vec<&str> = input.lines().collect();
    let mut input_lines = Vec::new();
    let mut extracted_map: HashMap<String, Vec<usize>> = HashMap::new();
    let mut valid_extractions = Vec::new();

    // 第一步：提取每行内容并标记错误行
    for (index, line) in lines.iter().enumerate() {
        let line_number = index + 1;
        let original_text = (*line).to_owned();

        if let Some(extracted) = extract_content_from_line(line) {
            // 成功提取
            extracted_map
                .entry(extracted.clone())
                .or_default()
                .push(line_number);

            input_lines.push(LineResult {
                line_number,
                original_text,
                extracted_content: Some(extracted.clone()),
                status: LineStatus::Normal,
            });

            valid_extractions.push(extracted);
        } else {
            // 提取失败，标记为错误
            input_lines.push(LineResult {
                line_number,
                original_text,
                extracted_content: None,
                status: LineStatus::Error,
            });
        }
    }

    // 第二步：标记重复项
    let mut duplicate_groups = HashMap::new();
    for (extracted, line_numbers) in &extracted_map {
        if line_numbers.len() > 1 {
            // 有重复，标记这些行
            for &line_num in line_numbers {
                if let Some(line_result) =
                    input_lines.iter_mut().find(|l| l.line_number == line_num)
                {
                    line_result.status = LineStatus::Duplicate;
                }
            }
            duplicate_groups.insert(extracted.clone(), line_numbers.clone());
        }
    }

    // 第三步：去重并排序输出
    let mut unique_extractions: Vec<String> = extracted_map.keys().cloned().collect();
    unique_extractions.sort();

    ProcessResult {
        input_lines,
        output_lines: unique_extractions,
        duplicate_groups,
    }
}

// Tauri 命令：处理文本
#[tauri::command]
fn process_text(input: String) -> Result<ProcessResult, String> {
    Ok(process_text_lines(&input))
}

// Tauri 命令：更新行选中状态
#[tauri::command]
fn toggle_line_selection(
    mut process_result: ProcessResult,
    extracted_content: String,
) -> Result<ProcessResult, String> {
    // 查找所有具有相同提取内容的行
    if let Some(line_numbers) = process_result.duplicate_groups.get(&extracted_content) {
        let line_numbers = line_numbers.clone();

        // 检查当前状态，如果已经是Selected则恢复为Duplicate，否则设为Selected
        let first_line = process_result
            .input_lines
            .iter()
            .find(|l| l.line_number == line_numbers[0])
            .unwrap();

        let new_status = match first_line.status {
            LineStatus::Selected => LineStatus::Duplicate,
            _ => LineStatus::Selected,
        };

        // 更新所有相关行的状态
        for line_number in line_numbers {
            if let Some(line_result) = process_result
                .input_lines
                .iter_mut()
                .find(|l| l.line_number == line_number)
            {
                line_result.status = new_status.clone();
            }
        }
    }

    Ok(process_result)
}

// Tauri 命令：导出结果到文件
#[tauri::command]
async fn export_to_file(_content: String) -> Result<String, String> {
    // 这里我们返回一个成功消息，实际的文件保存将在前端处理
    Ok("准备导出".to_owned())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            process_text,
            toggle_line_selection,
            export_to_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
