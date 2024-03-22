
// extern crate stopwatch;
use stopwatch::{Stopwatch};
use std::time::{Duration,SystemTime};
use std::fmt::Debug;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize,Ordering};

#[derive(Debug)]struct People {
    name: String,
}       

impl People {
    fn new(n :&str) -> People{
        People{
            name:n.to_string(),
        }
    }
    fn greet(&self){
        println!("{} say hello.",self.name);
    }
}

trait HasArea {
    fn area(&self) -> f64;
}

// trait里面的函数可以没有函数体，实现代码交给具体实现它的类型去补充：
#[derive(Debug)]struct Circle {
    x: f64,
    y: f64,
    radius:f64,
}   

impl HasArea for Circle{
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }   
}

// trait与泛型, 我们知道泛型可以指任意类型，但有时这不是我们想要的，需要给它一些约束。

// 单 trait 约束，
// Debug是Rust内置的一个trait，为"{:?}"实现打印内容，函数foo接受一个泛型作为参数，并且约定其需要实现Debuguse std::fmt::Debug;
fn foo<T: Debug>(s: T) {
    println!("{:?}",s);
}

// 多 trait 约束
fn foo1<T:Debug + Clone>(s: T) {
    s.clone();
    println!("{:?}",s);
}

// 泛型
// 在编程中，通常有这样的需求，为多种类型的数据编写一个功能相同的函数，
// 如两个数的加法，希望这个函数既支持i8、i16、 i32 ....float64等等，
// 甚至自定义类型，在不支持泛型的编程语言中，我们通常要为每一种类型都编写一个函数，
// 而且通常情况下函数名还必须不同

// 什么是闭包：闭包是引用了自由变量的函数。所以，闭包是一种特殊的函数。
// 在rust中，函数和闭包都是实现了Fn、FnMut或FnOnce特质（trait）的类型。
// 任何实现了这三种特质其中一种的类型的对象,都是可调用对象，
// 都能像函数和闭包一样通过这样name()的形式调用，()在rust中是一个操作符，
// 操作符在rust中是可以重载的。rust的操作符重载是通过实现相应的trait来实现，

// 1、基本形式

// let plus_one = |x:i32| x+1;
// assert_eq!(2, plus_one(1));

// let plus_two = |x| {
//     let mut result: i32 = x;
//     result += 1;
//     result += 1;
//     result
// };

// assert_eq!(4, plus_two(2));

// 错误处理// Option和Result
fn guss(n: i32) -> bool {
    if n< 1 || n> 10{
        panic!("Invalid number: {}",n);
    }
    n == 5
}   

fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}

// 宏定义 
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name(){
            println!("funciton{:?} is called",stringify!($func_name))
        }
    }
}

fn main() {
    let peter = People::new("Peter");
    peter.greet();

    let c = Circle{
        x:0.0_f64,
        y:0.0_f64,
        radius:1.0_f64,
    };
    println!("The circle area is {}",c.area());
    guss(5);

    let file_name = "foobar.rs";
    match find(file_name, '.') {
        None => println!("No file extension found."),
        Some(i) => println!("File extension: {}", &file_name[i+1..]),
    }

    create_function!(foo);
    foo();

    // 创建一个线程,法一
    let new_thread = thread::spawn(move || {
        println!("This is a new thread.");
    });

    new_thread.join().unwrap();     // 等待新创建的线程执行完成

    // 创建线程，法二。可设置线程的名称和堆栈大小(这里设置为 4k )
    let new_thread_result = thread::Builder::new()
                            .name("threadChen1".to_string())
                            .stack_size(4*1024*1024).spawn(move || {
                                println!("I am the threadChen1");
                            });
    new_thread_result.unwrap().join().unwrap();

    // 通道, 这里演示一个子线程和主进程之间通过通道通信的过程
    // create channel, 发送端和接收端 
    let (tx, rx) : (mpsc::Sender<&str>,mpsc::Receiver<&str>) = 
                    mpsc::channel();
     // 创建线程用于发送消息
     thread::spawn(move || {
         tx.send("Hello,Mr.chen. this's from channel").unwrap();
     });
    // 在主线程中接收子线程发送的消息并输出
    println!("receive: {}",rx.recv().unwrap());

    // 原子类型操作解决共享内存问题
    let var :Arc<AtomicUsize> = Arc::new(AtomicUsize::new(5));
    let shared_var = var.clone();
    
    // new thread 
    let new_thread = thread::spawn(move || {
        println!("shared value in new thread: {}",shared_var.load(Ordering::SeqCst));

        // change the num 
        shared_var.store(9,Ordering::SeqCst);
    });

    new_thread.join().unwrap();
    println!("shared value in main thread: {}",var.load(Ordering::SeqCst));
    let mut sum = 1;
    let sw = Stopwatch::start_new();
    let sy_time = SystemTime::now();
    for i in 1..10000 * 10 {
        // println!("i is {}",i);
        sum = sum + i;
    }
    println!("The sum is {},time is : {:?}",sum, sw.elapsed_ms());
    println!("Hello, world!");  sy_time.elapsed().unwrap().as_secs();
    println!("hello,world !");

}