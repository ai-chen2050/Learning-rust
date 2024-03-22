use self_hello::{
    hello_attribute, 
    show_streams,
    repeat
};

macro_rules! Hello {
    ($name:expr ) => {

        println!("Hello, {}!", $name);
    };
}

// 使用 hello_attribute 宏为函数添加注解
#[hello_attribute(Greet)]
fn greet() {}

#[show_streams(bar)]
fn invoke1() {}

#[repeat(2)]
fn prt() {
    println!("Hello, greet2!");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello, world!");
    Hello!("Kitty");
    greet();
    prt2();
    invoke1();
}

#[test]
fn feature() {
    let repeat_count = 2;
    let input_fn = "TokenStream::new()";
    if let Some(index) = input_fn.find('(') {
        for i in 0..repeat_count {
            let ret = format!("{}{}{}", &input_fn[..index], i+1, &input_fn[index..]);
            println!("fn: \"{}\"", ret);
        }
    }
}