/*
please see the ownership folder.
- string
- ownership
    - move
    - clone
    - ownership in function
        - return
        - parameter
        - dangling
- slice
- string literal
*/ 

fn main() {
    string_from_demo();

    move_demo1();
    move_demo2();

    clone_demo();
}

/// 1_string_demo.rs
fn string_from_demo() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // No push_str function:
    // "hello".push_str(", world!");

    // This will not pass compile
    // println!("{}", "hello" + ", world!");

    // Fix:
    println!("{}", "hello".to_owned() + &", world!");
}

/// 2_move_demo.rs
fn move_demo1() {
    let x = 5;
    let y = x;

    // Valid: Integer's size is known, so it will go to stack!
    println!("x={}, y={}", x, y);
}

fn move_demo2() {
    let s1 = String::from("hello");
    let s2 = s1;

    // Invalid: s1's value has been moved to s2!
    // println!("{}, world", s1);

    // Valid:
    println!("{}, world", s2);
}

/// 3. clone
fn clone_demo() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // Valid: s1's value has been copied!
    println!("{}, world", s1);

    // Valid:
    println!("{}, world", s2);
}