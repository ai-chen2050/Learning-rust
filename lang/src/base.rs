mod shape;
// extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

/*
 * Object Def Zone
*/

struct Circle{
    center: (i32, i32),
    radius: u32,
}  

impl Circle {
    fn new(center: (i32, i32), radius: u32) -> Circle {
        Circle {
            center: center,
            radius: radius,
        }
    }
     
    fn area(&self) -> f64 {
        // converting self.radius, a u32, to an f64.
        let f_radius = self.radius as f64;
         
        f_radius * f_radius * 3.14159
    }
     
    fn move_to(&mut self, new_center: (i32, i32)) {
        self.center = new_center;
    }
}

enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
 * Func Def Zone
*/


fn print_integer(value: i64){
    println!("The value is given : {}",value);
}

fn print_two_integer(value1: i32,value2: i64){
    println!("value1: {}, value2: {}",value1,value2);
}

fn increase_by_one(mut value: i32) -> i32 {
    value += 1;
    return value;
}

fn divmod(x: i32, y: i32) -> (i32, i32) {
    return (x/y , x%y);
}       

/*
 * Exec 
*/

fn main() {
    // 1、常见语法使用
    let mut x: i32 = 5;
    x = 6;
    let y: i64 = 8;
    let mut z: i32 = 10;
    print_integer(y);
    print_two_integer(x, y);
    println!("Before: x is = {}",x);
    x = increase_by_one(x);
    println!("After: x is = {}",x);
    // x, z = divmod(x, z);
    println!("After Divmoding x = {}, z = {}", x, z);
    let (w,z) = (3, "hello,chen");
    println!("hello,Mr.chen.");

    // 2、字符串
    let  str1 = "hello,world !";
    let str2 : &str = "This is rust's trait";
    let str3 = String::from("hello,world2");
    let str4: String = String::from("This is rust's trait_4.");
    println!("{}",str4);

    // 3、数组 
    let array1 = [1, 2, 3];
    let array2 : [i32;2] = [4, 5];
    let s :usize = array1.len();    // s == 3
    let e = array1[1];
    let vec1 = vec![1, 2, 3];
    let y: Vec<i32> = [4, 5, 6].to_vec();

    // 4、tuple 
    let x = (5, '6');
    let y: (i32, char) = (7, '8');
    let z0 = y.0;
    let z1 = y.1;

    // 5、control stream
    let x = '5';
    if x == '5' {
        println!("X is the char '5'! ");
    }else if x == '6'{
        println!("X is the char '6'! ");
    }else {
        println!("I don't know what X is.");
    }

    // ternary form (三元不等式)
    let y = if x == '5' { 5 } else if x == '6' { 6 } else { -1 };

    // 6、cycle loop  ==> loop、for 和 while
    let mut x = 0;
    while x != 5 {
        println!("x :{} ", x);
        x += 1;         // this is equal to x = x + 1;
    }

    for x in 0..5{
        println!("x: {}", x);
    }

    let x = vec![0, 1, 2, 3, 4];
    for element in &x{
        println!("element: {}",element);
    }

    // 7、movable semantic
    let x = String::from("Hello, World!");
     
    println!("{}", x); // x is valid here
 
    let y = x;   // the resource assigned to x is move to y
     
    // println!("{}", x);  Error

    // 8、struct 
    let c_1 = Circle {
        center: (-1,1),
        radius: 1,
    };

    let c_1 = Circle {
        center: (-1,1),
        radius: 1,
    };

    println!("c_1's radius is {}", c_1.radius);

    let mut c = Circle::new((0,0), 1);
    println!( "The circle’s area is {}" , c.area());
    println!( "The circles location is {:?}" , c.center);

    c.move_to((-1,1));
    println!("The circles location is {:?}", c.center);

    // Rust 中的方法有 3 个唯一变量：self、&self、&mut self。
    // 回想一下关于所有权和借用的所有知识，这些变量分别将对象所有权转移给该方法，
    // 不可变地借用一个对象，再可变地借用一个对象。
    
    // 9、enum 
    let red = Color::Red;
    let blue :Color = Color::Blue;

    // 10、match 是一个表达式
    let x = 5;
     
    match x {
        1 => println!("“Matched to 1!”"),
        2 => println!("“Matched to 2!”"),
        3 => println!("“Matched to 3!”"),
        4 => println!("“Matched to 4!”"),
        5 => println!("“Matched to 5!”"),
        6 => println!("“Matched to 6!”"),
        _ => println!("“Matched to some other number!”"),
    };

    // 11、mod 
    let c = shape::Circle{
        center:(0,0),
        radius: 1,
    };

    // 12、use 
    let c2 = Circle{
        center:(0,0),
        radius: 1,
    };

    use std::collections::HashMap;
    let mut counter = HashMap::new();
    counter.insert('a',1);

    use rand::Rng;
    let mut rng = rand::thread_rng();
    let x = rng.gen::<i32>();

    print!("x: {}",x);

    // 13、dbg!  rectangle
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("The rectangle is {:#?}.", rect1);

    // 14 guess the number
    let open = false;
    if open {
        println!("Guess the number");
        let secret_num = rand::thread_rng().gen_range(1..=100);
    
        println!("The secret number is: {}", secret_num);
        loop {
            println!("Input your guess.");
            let mut guess = String::new();
    
            io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
            let guess : u32 =  match guess.trim().parse() {     //.expect("Please input a number !");
                Ok(num) => num,
                Err(_) => {
                    println!("Please input a number!\n");
                    continue;
                }
            };
    
            println!("You guessed: {}", guess);
            match guess.cmp(&secret_num) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }  
    }

}

