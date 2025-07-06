use zed_extension_api::{self as zed, Result};
use std::collections::HashMap;

struct CodeLensExtension {
    symbol_counts: HashMap<String, HashMap<String, u32>>,
}

impl CodeLensExtension {
    fn new() -> Self {
        Self {
            symbol_counts: HashMap::new(),
        }
    }

    fn analyze_file(&mut self, file_path: &str, content: &str) -> Result<()> {
        let counts = self.count_symbol_references(content, file_path);
        self.symbol_counts.insert(file_path.to_string(), counts);
        Ok(())
    }

    fn count_symbol_references(&self, content: &str, file_path: &str) -> HashMap<String, u32> {
        let mut counts = HashMap::new();
        
        // Determine file type by extension
        let is_python = file_path.ends_with(".py");
        let is_js_ts = file_path.ends_with(".js") || file_path.ends_with(".ts") || file_path.ends_with(".tsx") || file_path.ends_with(".jsx");
        
        if is_python {
            self.analyze_python_symbols(content, &mut counts);
        } else if is_js_ts {
            self.analyze_js_ts_symbols(content, &mut counts);
        }
        
        counts
    }

    fn analyze_python_symbols(&self, content: &str, counts: &mut HashMap<String, u32>) {
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // Function definitions
            if trimmed.starts_with("def ") {
                if let Some(func_name) = self.extract_python_function_name(line) {
                    let count = self.count_symbol_usage(&func_name, content, line_num);
                    counts.insert(func_name, count);
                }
            }
            // Class definitions
            else if trimmed.starts_with("class ") {
                if let Some(class_name) = self.extract_python_class_name(line) {
                    let count = self.count_symbol_usage(&class_name, content, line_num);
                    counts.insert(class_name, count);
                }
            }
            // Variable assignments
            else if trimmed.contains(" = ") && !trimmed.starts_with("#") {
                if let Some(var_name) = self.extract_python_variable_name(line) {
                    let count = self.count_symbol_usage(&var_name, content, line_num);
                    counts.insert(var_name, count);
                }
            }
        }
    }

    fn analyze_js_ts_symbols(&self, content: &str, counts: &mut HashMap<String, u32>) {
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // Function declarations
            if trimmed.starts_with("function ") {
                if let Some(func_name) = self.extract_function_name(line) {
                    let count = self.count_symbol_usage(&func_name, content, line_num);
                    counts.insert(func_name, count);
                }
            }
            // Variable declarations
            else if trimmed.starts_with("const ") || trimmed.starts_with("let ") || trimmed.starts_with("var ") {
                if let Some(var_name) = self.extract_variable_name(line) {
                    let count = self.count_symbol_usage(&var_name, content, line_num);
                    counts.insert(var_name, count);
                }
            }
            // Class declarations
            else if trimmed.starts_with("class ") {
                if let Some(class_name) = self.extract_js_class_name(line) {
                    let count = self.count_symbol_usage(&class_name, content, line_num);
                    counts.insert(class_name, count);
                }
            }
            // Interface declarations (TypeScript)
            else if trimmed.starts_with("interface ") {
                if let Some(interface_name) = self.extract_ts_interface_name(line) {
                    let count = self.count_symbol_usage(&interface_name, content, line_num);
                    counts.insert(interface_name, count);
                }
            }
            // Type declarations (TypeScript)
            else if trimmed.starts_with("type ") {
                if let Some(type_name) = self.extract_ts_type_name(line) {
                    let count = self.count_symbol_usage(&type_name, content, line_num);
                    counts.insert(type_name, count);
                }
            }
        }
    }

    fn extract_function_name(&self, line: &str) -> Option<String> {
        // Extract function name from "function functionName(" pattern
        let trimmed = line.trim();
        if let Some(start) = trimmed.find("function ") {
            let after_function = &trimmed[start + 9..];
            if let Some(end) = after_function.find('(') {
                let name = after_function[..end].trim();
                if !name.is_empty() {
                    return Some(name.to_string());
                }
            }
        }
        None
    }

    fn extract_variable_name(&self, line: &str) -> Option<String> {
        // Extract variable name from "const/let/var varName =" pattern
        let trimmed = line.trim();
        let keywords = ["const ", "let ", "var "];
        
        for keyword in keywords {
            if let Some(start) = trimmed.find(keyword) {
                let after_keyword = &trimmed[start + keyword.len()..];
                if let Some(end) = after_keyword.find(' ') {
                    let name = after_keyword[..end].trim();
                    if !name.is_empty() {
                        return Some(name.to_string());
                    }
                }
            }
        }
        None
    }

    fn extract_python_function_name(&self, line: &str) -> Option<String> {
        // Extract function name from "def function_name(" pattern
        let trimmed = line.trim();
        if let Some(start) = trimmed.find("def ") {
            let after_def = &trimmed[start + 4..];
            if let Some(end) = after_def.find('(') {
                let name = after_def[..end].trim();
                if !name.is_empty() {
                    return Some(name.to_string());
                }
            }
        }
        None
    }

    fn count_symbol_usage(&self, symbol_name: &str, content: &str, definition_line: usize) -> u32 {
        let mut count = 0;
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            // Skip the definition line
            if line_num == definition_line {
                continue;
            }
            
            // Count occurrences in the line
            let mut start = 0;
            while let Some(pos) = line[start..].find(symbol_name) {
                let actual_pos = start + pos;
                let is_word_boundary = self.is_word_boundary(line, actual_pos, symbol_name.len());
                
                if is_word_boundary {
                    count += 1;
                }
                start = actual_pos + 1;
            }
        }
        
        count
    }

    fn is_word_boundary(&self, line: &str, pos: usize, len: usize) -> bool {
        let chars: Vec<char> = line.chars().collect();
        
        // Check character before
        let before_ok = pos == 0 || !chars[pos - 1].is_alphanumeric() && chars[pos - 1] != '_';
        
        // Check character after
        let after_ok = pos + len >= chars.len() || !chars[pos + len].is_alphanumeric() && chars[pos + len] != '_';
        
        before_ok && after_ok
    }

    fn extract_python_class_name(&self, line: &str) -> Option<String> {
        let trimmed = line.trim();
        if let Some(start) = trimmed.find("class ") {
            let after_class = &trimmed[start + 6..];
            let end = after_class.find('(').unwrap_or_else(|| after_class.find(':').unwrap_or(after_class.len()));
            let name = after_class[..end].trim();
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
        None
    }

    fn extract_python_variable_name(&self, line: &str) -> Option<String> {
        let trimmed = line.trim();
        if let Some(eq_pos) = trimmed.find(" = ") {
            let before_eq = &trimmed[..eq_pos];
            // Handle simple variable assignments (not tuple unpacking)
            if !before_eq.contains(',') && !before_eq.contains('(') && !before_eq.contains('[') {
                let name = before_eq.trim();
                if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    return Some(name.to_string());
                }
            }
        }
        None
    }

    fn extract_js_class_name(&self, line: &str) -> Option<String> {
        let trimmed = line.trim();
        if let Some(start) = trimmed.find("class ") {
            let after_class = &trimmed[start + 6..];
            let end = after_class.find('{').unwrap_or_else(|| after_class.find(' ').unwrap_or(after_class.len()));
            let name = after_class[..end].trim();
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
        None
    }

    fn extract_ts_interface_name(&self, line: &str) -> Option<String> {
        let trimmed = line.trim();
        if let Some(start) = trimmed.find("interface ") {
            let after_interface = &trimmed[start + 10..];
            let end = after_interface.find('{').unwrap_or_else(|| after_interface.find(' ').unwrap_or(after_interface.len()));
            let name = after_interface[..end].trim();
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
        None
    }

    fn extract_ts_type_name(&self, line: &str) -> Option<String> {
        let trimmed = line.trim();
        if let Some(start) = trimmed.find("type ") {
            let after_type = &trimmed[start + 5..];
            let end = after_type.find('=').unwrap_or_else(|| after_type.find(' ').unwrap_or(after_type.len()));
            let name = after_type[..end].trim();
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
        None
    }
}

impl zed::Extension for CodeLensExtension {
    fn new() -> Self {
        Self::new()
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // This extension doesn't provide a language server
        // Instead, it hooks into Zed's inlay hints system
        Ok(zed::Command {
            command: "echo".to_string(),
            args: vec!["CodeLens extension loaded".to_string()],
            env: Default::default(),
        })
    }

    fn label_for_completion(
        &self,
        _language_server_id: &zed::LanguageServerId,
        _completion: zed::lsp::Completion,
    ) -> Option<zed::CodeLabel> {
        // Not used for this extension
        None
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &zed::LanguageServerId,
        _symbol: zed::lsp::Symbol,
    ) -> Option<zed::CodeLabel> {
        // This could be used to show reference counts in symbol outlines
        // For now, we'll return None and focus on inlay hints
        None
    }
}

zed::register_extension!(CodeLensExtension);