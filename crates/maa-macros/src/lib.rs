mod task;
mod utils;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// 为结构体自动实现 Task trait 和 Builder 模式
#[proc_macro_derive(GenerateTask, attributes(task))]
pub fn derive_generate_task(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = task::generate_task(input).unwrap_or_else(|err| err.to_compile_error());
    TokenStream::from(expanded)
}
