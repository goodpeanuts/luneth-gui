// Tauri 命令：处理文本
#[tauri::command]
pub(crate) fn process_text(input: &str) -> ProcessResult {
    log::debug!("Processing text input with {} characters", input.len());
    let lines_count = input.lines().count();
    log::debug!("Input contains {lines_count} lines");

    let result = process_text_lines(input);

    log::info!(
        "Text processing completed: {} valid extractions, {} duplicate groups, {} error lines",
        result.output_lines.len(),
        result.duplicate_groups.len(),
        result
            .input_lines
            .iter()
            .filter(|l| matches!(l.status, LineStatus::Error))
            .count()
    );

    result
}

// Tauri 命令：更新行选中状态
#[tauri::command]
pub fn toggle_line_selection(
    mut process_result: ProcessResult,
    extracted_content: &str,
) -> ProcessResult {
    log::debug!("Toggling selection for extracted content: {extracted_content}");

    // 查找所有具有相同提取内容的行
    if let Some(line_numbers) = process_result.duplicate_groups.get(extracted_content) {
        let line_numbers = line_numbers.clone();
        log::debug!(
            "Found {} lines with content '{}'",
            line_numbers.len(),
            extracted_content
        );

        // 检查当前状态，如果已经是Selected则恢复为Duplicate，否则设为Selected
        let Some(first_line) = process_result
            .input_lines
            .iter()
            .find(|l| l.line_number == line_numbers[0])
        else {
            log::warn!("Could not find line with number: {}", line_numbers[0]);
            return process_result;
        };

        let new_status = match first_line.status {
            LineStatus::Selected => LineStatus::Duplicate,
            _ => LineStatus::Selected,
        };

        log::debug!("Changing status to: {new_status:?}");

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
    } else {
        log::debug!("No duplicate group found for content: {extracted_content}");
    }

    process_result
}

// Tauri 命令：导出结果到文件
#[tauri::command]
pub(crate) async fn export_to_file(content: String) -> Result<String, String> {
    log::debug!(
        "Export request received for content with {} characters",
        content.len()
    );
    log::debug!(
        "Export content preview: {}...",
        if content.len() > 100 {
            &content[..100]
        } else {
            &content
        }
    );

    // 这里我们返回一个成功消息，实际的文件保存将在前端处理
    Ok("准备导出".to_owned())
}

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
    let email_re = Regex::new(r"([a-zA-Z]+)-(\d+)").expect("Invalid regex");
    if let Some(mat) = email_re.find(line) {
        return Some(mat.as_str().to_owned());
    }

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
    #[expect(clippy::iter_over_hash_type)]
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
