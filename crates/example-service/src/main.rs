//! Entry point of the example program.
//!
//! 示例程序入口。

fn main() {
    println!("{}", example_service::banner(env!("CARGO_PKG_NAME")));
}
