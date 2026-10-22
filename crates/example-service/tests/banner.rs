//! Integration tests for the public `banner` API.
//!
//! 针对公开 `banner` 接口的集成测试。

#[test]
fn banner_uses_provided_name() {
    assert_eq!(
        example_service::banner("example-service"),
        "Hello, example-service!"
    );
}
