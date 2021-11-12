
// 下面这个宏将每一个元素都转换成字符串。它将匹配零或多个由逗号分隔的表达式，并分别将它们拓展成构造 Vec 的表达式。
macro_rules! vec_strs {
    (
        // Start a repetition:
        $(
            // Each repeat must contain an expression...
            $element: expr
        )
        // ...separated by commas...
        ,
        // ...zero or more times.
        *
    ) => {
        // Enclose the expansion in a block so that we can use multiple statements. 
        {
            let mut v = Vec::new();

            // Start a repetition:
            $(
                // Each repeat will contain the following statement,
                // with the corresponding expression. 
                v.push(format!("{}", $element));
            )*
            v
        }
    };
}

fn main() {
    let s = vec_strs![1, "a", true, 1+1, 3.14159f32];
    println!("{:?}", s);
    // assert_eq!(s, &["1", "a", "true", "3.14159"]);
}
