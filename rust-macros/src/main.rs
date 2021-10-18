extern crate proc_macro_examples;
use proc_macro_examples::{make_answer, show_streams};

make_answer!();

// 基础函数
#[show_streams]
fn invoke1() {}

// 带输入参数的属性
#[show_streams(bar)]
fn invoke2() {}

// 输入参数中有多个token
#[show_streams(multiple => tokens)]
fn invoke3() {}

#[show_streams { delimiters }]
fn invoke4() {}



fn main() {
    println!("{}", answer());
}
