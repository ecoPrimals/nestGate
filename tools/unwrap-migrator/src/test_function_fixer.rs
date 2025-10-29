//! # Test Function Signature Fixer
//!
//! Automatically fixes test function signatures when using SafeUnwrap with ? operator.
//!
//! ## Patterns Fixed
//!
//! 1. Missing return type when using `?` operator with `safe_unwrap`
//! 2. Incorrect `Result<(), NestGateError>` (2 generics) → `crate::Result<()>` (1 generic)
//! 3. Missing `Ok(())` at function end
//! 4. Missing imports in test module
//!
//! ## Example
//!
//! ```rust,ignore
//! // BEFORE:
//! #[test]
//! fn test_something() {
//!     let value = operation().safe_unwrap(ErrorCategory::Something, "context")?;
//!     assert_eq!(value, expected);
//! }
//!
//! // AFTER:
//! #[test]
//! fn test_something() -> crate::Result<()> {
//!     let value = operation().safe_unwrap(ErrorCategory::Something, "context")?;
//!     assert_eq!(value, expected);
//!     Ok(())
//! }
//! ```

use regex::Regex;
use std::path::Path;
use tokio::fs;
use tracing::info;

#[derive(Debug, Clone)]
pub struct TestFunctionFix {
    pub file_path: String,
    pub line_number: usize,
    pub function_name: String,
    pub fix_type: TestFixType,
    pub original: String,
    pub fixed: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestFixType {
    /// Add return type `-> crate::Result<()>`
    AddReturnType,
    /// Fix incorrect return type `Result<(), NestGateError>` → `crate::Result<()>`
    FixReturnType,
    /// Add `Ok(())` at end of function
    AddOkReturn,
    /// Add imports to test module
    AddImports,
    /// Combined fix (multiple changes)
    Combined,
}

pub struct TestFunctionFixer {
    /// Pattern to detect test functions
    test_attr_pattern: Regex,
    /// Pattern to detect safe_unwrap with ? operator
    safe_unwrap_question_pattern: Regex,
    /// Pattern to detect function signature
    function_sig_pattern: Regex,
    /// Pattern to detect Result<(), NestGateError>
    incorrect_result_pattern: Regex,
    /// Pattern to detect existing imports
    import_pattern: Regex,
    /// Statistics
    fixes_applied: usize,
    files_modified: usize,
}

impl TestFunctionFixer {
    pub fn new() -> Self {
        Self {
            test_attr_pattern: Regex::new(r"#\[(test|tokio::test|serial_test::serial)\]").unwrap(),
            safe_unwrap_question_pattern: Regex::new(r"\.safe_unwrap\([^)]+\)\?").unwrap(),
            function_sig_pattern: Regex::new(r"(async\s+)?fn\s+(\w+)\s*\([^)]*\)\s*(->.*?)?\s*\{")
                .unwrap(),
            incorrect_result_pattern: Regex::new(r"Result<\(\),\s*NestGateError>").unwrap(),
            import_pattern: Regex::new(r"use\s+crate::error::\{[^}]*ErrorCategory[^}]*\}").unwrap(),
            fixes_applied: 0,
            files_modified: 0,
        }
    }

    /// Analyze a file for test functions needing fixes
    pub async fn analyze_file(&self, path: &Path) -> Result<Vec<TestFunctionFix>, std::io::Error> {
        let content = fs::read_to_string(path).await?;
        let mut fixes = Vec::new();

        // Split into lines for analysis
        let lines: Vec<&str> = content.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            // Check if this is a test attribute
            if self.test_attr_pattern.is_match(line) {
                // Check the next few lines for the function and safe_unwrap usage
                if let Some(fix) = self.analyze_test_function(&lines, idx, path) {
                    fixes.push(fix);
                }
            }
        }

        // Check if test module needs imports
        if !fixes.is_empty() && !self.has_required_imports(&content) {
            fixes.push(TestFunctionFix {
                file_path: path.to_string_lossy().to_string(),
                line_number: 0,
                function_name: String::from("test module"),
                fix_type: TestFixType::AddImports,
                original: String::new(),
                fixed: String::from("use crate::error::{ErrorCategory, SafeUnwrap};"),
            });
        }

        Ok(fixes)
    }

    /// Analyze a specific test function
    fn analyze_test_function(
        &self,
        lines: &[&str],
        test_attr_line: usize,
        path: &Path,
    ) -> Option<TestFunctionFix> {
        // Get the function signature (should be next non-empty line after #[test])
        let mut func_line_idx = test_attr_line + 1;
        while func_line_idx < lines.len() && lines[func_line_idx].trim().is_empty() {
            func_line_idx += 1;
        }

        if func_line_idx >= lines.len() {
            return None;
        }

        let func_line = lines[func_line_idx];
        let func_sig_captures = self.function_sig_pattern.captures(func_line)?;

        let function_name = func_sig_captures.get(2)?.as_str().to_string();
        let existing_return = func_sig_captures.get(3).map(|m| m.as_str());

        // Check if function body uses safe_unwrap with ?
        let mut has_safe_unwrap_question = false;
        let mut func_end_line = func_line_idx + 1;
        let mut brace_count = 1; // Start with opening brace

        while func_end_line < lines.len() && brace_count > 0 {
            let line = lines[func_end_line];
            if self.safe_unwrap_question_pattern.is_match(line) {
                has_safe_unwrap_question = true;
            }
            brace_count += line.matches('{').count() as i32;
            brace_count -= line.matches('}').count() as i32;
            func_end_line += 1;
        }

        if !has_safe_unwrap_question {
            return None; // No fix needed if not using safe_unwrap with ?
        }

        // Determine what fix is needed
        let fix_type = match existing_return {
            None => TestFixType::AddReturnType,
            Some(ret) if self.incorrect_result_pattern.is_match(ret) => TestFixType::FixReturnType,
            Some(ret) if ret.contains("Result<(), NestGateError>") => TestFixType::FixReturnType,
            Some(ret) if !ret.contains("Result") => TestFixType::AddReturnType,
            _ => return None, // Already correct
        };

        // Check if function already ends with Ok(())
        let last_significant_line =
            self.find_last_significant_line(lines, func_line_idx, func_end_line);
        let needs_ok_return = last_significant_line
            .map(|line| !line.trim().starts_with("Ok(())"))
            .unwrap_or(true);

        let fix_type = if needs_ok_return && fix_type != TestFixType::AddReturnType {
            TestFixType::Combined
        } else {
            fix_type
        };

        // Build the fix
        let original = func_line.to_string();
        let fixed = self.build_fixed_signature(func_line, &fix_type);

        Some(TestFunctionFix {
            file_path: path.to_string_lossy().to_string(),
            line_number: func_line_idx + 1,
            function_name,
            fix_type,
            original,
            fixed,
        })
    }

    /// Find the last significant line (non-empty, non-brace) in function
    fn find_last_significant_line<'a>(
        &self,
        lines: &[&'a str],
        start: usize,
        end: usize,
    ) -> Option<&'a str> {
        for idx in (start..end).rev() {
            let line = lines[idx].trim();
            if !line.is_empty() && line != "}" && line != "{" {
                return Some(line);
            }
        }
        None
    }

    /// Build fixed function signature
    fn build_fixed_signature(&self, original: &str, fix_type: &TestFixType) -> String {
        match fix_type {
            TestFixType::AddReturnType => {
                // Add -> crate::Result<()> before {
                original
                    .replace(" {", " -> crate::Result<()> {")
                    .replace("){", ") -> crate::Result<()> {")
            }
            TestFixType::FixReturnType => {
                // Replace Result<(), NestGateError> with crate::Result<()>
                let fixed = self
                    .incorrect_result_pattern
                    .replace_all(original, "crate::Result<()>")
                    .to_string();
                fixed.replace("Result<(), NestGateError>", "crate::Result<()>")
            }
            TestFixType::Combined | TestFixType::AddOkReturn => {
                // First fix the return type if needed
                if original.contains("Result<(), NestGateError>") {
                    original.replace("Result<(), NestGateError>", "crate::Result<()>")
                } else if !original.contains(" -> ") {
                    original
                        .replace(" {", " -> crate::Result<()> {")
                        .replace("){", ") -> crate::Result<()> {")
                } else {
                    original.to_string()
                }
            }
            TestFixType::AddImports => original.to_string(),
        }
    }

    /// Check if file has required imports
    fn has_required_imports(&self, content: &str) -> bool {
        self.import_pattern.is_match(content)
            || (content.contains("use crate::error::ErrorCategory")
                && content.contains("use crate::error::SafeUnwrap"))
    }

    /// Apply fixes to a file
    pub async fn apply_fixes(
        &mut self,
        path: &Path,
        fixes: &[TestFunctionFix],
    ) -> Result<usize, std::io::Error> {
        if fixes.is_empty() {
            return Ok(0);
        }

        let content = fs::read_to_string(path).await?;
        let mut new_content = content.clone();
        let mut applied = 0;

        // First, apply imports if needed
        for _fix in fixes
            .iter()
            .filter(|f| f.fix_type == TestFixType::AddImports)
        {
            if !self.has_required_imports(&new_content) {
                // Find the test module start
                if let Some(test_mod_pos) = new_content.find("#[cfg(test)]") {
                    if let Some(mod_tests_pos) = new_content[test_mod_pos..].find("mod tests") {
                        if let Some(use_super_pos) =
                            new_content[test_mod_pos + mod_tests_pos..].find("use super::*;")
                        {
                            let insert_pos = test_mod_pos
                                + mod_tests_pos
                                + use_super_pos
                                + "use super::*;".len();
                            new_content.insert_str(
                                insert_pos,
                                "\n    use crate::error::{ErrorCategory, SafeUnwrap};",
                            );
                            applied += 1;
                            info!("✅ Added imports to test module in {}", path.display());
                        }
                    }
                }
            }
        }

        // Then apply function signature fixes
        for fix in fixes
            .iter()
            .filter(|f| f.fix_type != TestFixType::AddImports)
        {
            if new_content.contains(&fix.original) {
                new_content = new_content.replace(&fix.original, &fix.fixed);
                applied += 1;
                info!(
                    "✅ Fixed test function {} in {}",
                    fix.function_name,
                    path.display()
                );
            }
        }

        // Apply Ok(()) additions
        for fix in fixes
            .iter()
            .filter(|f| matches!(f.fix_type, TestFixType::AddOkReturn | TestFixType::Combined))
        {
            // Find the function and add Ok(()) before the closing brace
            // This is a simplified approach - in production you'd want more robust parsing
            if let Some(func_pos) = new_content.find(&format!("fn {}", fix.function_name)) {
                // Find the closing brace of this function
                // This is simplified - you'd want proper brace matching
                let remaining = &new_content[func_pos..];
                if let Some(closing_brace) = Self::find_function_closing_brace(remaining) {
                    let absolute_pos = func_pos + closing_brace;
                    // Insert Ok(()) before the closing brace with proper indentation
                    let before_brace = &new_content[..absolute_pos];
                    let indent = Self::get_indent_from_context(before_brace);
                    new_content.insert_str(
                        absolute_pos,
                        &format!(
                            "\n{}Ok(())\n{}",
                            indent,
                            &indent[..indent.len().saturating_sub(4)]
                        ),
                    );
                    info!(
                        "✅ Added Ok(()) to {} in {}",
                        fix.function_name,
                        path.display()
                    );
                }
            }
        }

        // Write the modified content
        if applied > 0 {
            fs::write(path, new_content).await?;
            self.fixes_applied += applied;
            self.files_modified += 1;
        }

        Ok(applied)
    }

    /// Find the closing brace of a function (simplified)
    fn find_function_closing_brace(content: &str) -> Option<usize> {
        let mut brace_count = 0;
        let mut found_opening = false;

        for (idx, ch) in content.char_indices() {
            match ch {
                '{' => {
                    brace_count += 1;
                    found_opening = true;
                }
                '}' => {
                    if found_opening {
                        brace_count -= 1;
                        if brace_count == 0 {
                            return Some(idx);
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Get indentation from context
    fn get_indent_from_context(content: &str) -> String {
        // Find the last line with content
        if let Some(last_line_start) = content.rfind('\n') {
            let last_line = &content[last_line_start + 1..];
            let indent_count = last_line.chars().take_while(|c| c.is_whitespace()).count();
            " ".repeat(indent_count)
        } else {
            "    ".to_string()
        }
    }

    /// Get statistics
    pub fn stats(&self) -> (usize, usize) {
        (self.fixes_applied, self.files_modified)
    }
}

impl Default for TestFunctionFixer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_detection() {
        let fixer = TestFunctionFixer::new();

        // Test that patterns compile
        assert!(fixer.test_attr_pattern.is_match("#[test]"));
        assert!(fixer.test_attr_pattern.is_match("#[tokio::test]"));
        assert!(fixer
            .safe_unwrap_question_pattern
            .is_match(".safe_unwrap(ErrorCategory::Test, \"test\")?"));
    }

    #[test]
    fn test_incorrect_result_pattern() {
        let fixer = TestFunctionFixer::new();
        assert!(fixer
            .incorrect_result_pattern
            .is_match("Result<(), NestGateError>"));
        assert!(!fixer.incorrect_result_pattern.is_match("crate::Result<()>"));
    }

    #[test]
    fn test_build_fixed_signature() {
        let fixer = TestFunctionFixer::new();

        let original = "    fn test_something() {";
        let fixed = fixer.build_fixed_signature(original, &TestFixType::AddReturnType);
        assert_eq!(fixed, "    fn test_something() -> crate::Result<()> {");
    }
}
