fn main() {
    println!("Hello, world!");
    fun_array();
}

fn fun1() {
    let x = 10;
    println!("x = {}", x);
    let mut y = 20;
    println!("y = {}", y);
    y = 30;
    println!("y = {}", y);
}

fn fun2() {
    let mut i1:i64 = 2147483647; //std::i32::MAX;
    println!("il = {}", i1);
    i1 = i1 + 1;
    println!("il = {}", i1);
}

fn fun_tup() {
    let tup: (i32, u8, f32) = (10, 20, 2.5);
    let (x, y, z) = tup;
    let i = tup.0;
    let u = tup.1;
    let f = tup.2;
    println!("i = {}", i);
    println!("u = {}", u);
    println!("f = {}", f);
}

fn fun_array() {
    let arr = [10, 20, 30, 40, 50];
    println!("1st Element = {}", arr[0]);
    println!("10th Element = {}", arr[9]);
}

