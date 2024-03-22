use proc_macro::TokenStream;

// 定义一个属性宏 hello_attribute，将传入的字符串包装在 println! 宏中
#[proc_macro_attribute]
pub fn hello_attribute(attr: TokenStream, input1: TokenStream) -> TokenStream {
    let output = format!("println!(\"Hello, {}\");", attr.to_string());
    println!("{}", output);
    input1
}

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro_attribute]
pub fn repeat(attr: TokenStream, input: TokenStream) -> TokenStream {
    let repeat_count = attr.to_string().parse::<u32>().unwrap();
    let input_fn = input.to_string();
    // println!("fn: \"{}\"", input_fn);

    let mut output = TokenStream::new();
    if let Some(index) = input_fn.find('(') {
        for i in 0..repeat_count {
            let ret = format!("{}{}{}", &input_fn[..index], i+1, &input_fn[index..]);
            println!("fn: \"{}\"", ret);
            output.extend(ret.parse::<TokenStream>().unwrap());
        }
    }

    output
}