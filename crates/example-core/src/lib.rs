//! Shared example library logic for reuse by other crates in the workspace.
//!
//! 通用示例库逻辑，供工作区内其他 crate 复用。

/// Generates a greeting for use by other crates in the workspace.
///
/// 生成一条问候语，供工作区内其他 crate 调用。
///
/// # Examples
///
/// 示例：
///
/// ```
/// assert_eq!(example_core::greet("world"), "Hello, world!");
/// ```
#[must_use]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_handles_empty_name() {
        assert_eq!(greet(""), "Hello, !");
    }

    #[test]
    fn greet_preserves_unicode() {
        assert_eq!(greet("世界"), "Hello, 世界!");
    }
}
