extern crate proc_macro_examples;
use proc_macro_examples::{make_answer, show_streams, HlperAttr, flaky_test};

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

#[derive(HlperAttr, Debug)]
struct Struct {
    #[helper] field: ()
}

#[flaky_test]
fn my_test() {
    assert_eq!(1, 1);
}


fn main() {
    println!("{}", answer());
    let struct1 = Struct { field: () };
    println!("{:?}", struct1);
}
