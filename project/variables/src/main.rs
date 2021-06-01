fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let mut y = x;
    y = 100;
    println!("x = {}", x);

    const MAX_POINTS: u32 = 100_000;

    println!("max points = {}", MAX_POINTS);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess= {}", guess);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x = {}", x);
    println!("y = {}", y);

    // 加法
    let sum = 5 + 10;
    // 减法
    let difference = 95.5 - 4.3;
    // 乘法
    let product = 4 * 30;
    // 除法
    let quotient = 56.7 / 32.2;
    // 取余
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; // 显式指定类型注解

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("happy children day:{}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup:{0} , {1}, {2}", tup.0, tup.1, tup.2);


    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    // println!("months:{}", months)

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[0]:{}", a[0]);


    let a = [1, 2, 3, 4, 5];
    let index = 3;
    let element = a[index];
    println!("The value of element is: {}", element);
    another_function2(5);
    another_function3(5, 6);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    println!("five() = {}", five());

    println!("plus_one() = {}", plus_one(5));

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function3(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}