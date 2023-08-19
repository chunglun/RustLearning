fn main() {
    //println!("Hello, world!");
    fun_for();

}

fn fun_for() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("element = {}", element);
    }

    let mut index = 0;
    while index < 5 {
        println!("element = {} ..", a[index]);
        index += 1;
    }
}

fn fun_loop() {
    loop {
        println!("looping !");
        break;
    }

    let mut n = 5;
    while n != 0 {
        println!("n = {}", n);
        n -= 1;
    }
}

fn fun_if() {
    let condition = true;
    let number = if condition {
        5
    } else {
        10
    };
    println!("number = {}", number);
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

fn fun_expression() {
    let y= {
        let x = 10;
        x+1
    };
    println!("y = {}", y);
}

fn sum(x:i32, y:i32) -> i32 {
    x+y
}

fn sum1000(x:i32, y:i32) -> i32 {
    let x= x+y;
    x*1000
}