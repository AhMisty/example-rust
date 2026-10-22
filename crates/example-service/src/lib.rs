//! Example program logic used by the binary entry point.
//!
//! 示例程序逻辑，供二进制入口调用。

/// Returns the banner printed when the example program starts for the given name.
///
/// 返回示例程序针对给定名称启动时打印的横幅文本。
///
/// # Examples
///
/// 示例：
///
/// ```
/// assert_eq!(example_service::banner("world"), "Hello, world!");
/// ```
#[must_use]
pub fn banner(name: &str) -> String {
    example_core::greet(name)
}
