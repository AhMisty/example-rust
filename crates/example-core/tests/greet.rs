//! Integration tests for the public `greet` API.
//!
//! 针对公开 `greet` 接口的集成测试。

#[test]
fn greet_uses_provided_name() {
    assert_eq!(example_core::greet("example"), "Hello, example!");
}
