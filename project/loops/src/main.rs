fn main() {
    // loop {
    //     println!("again!");
    // }
    // let mut counter = 0;
    //
    // let result = loop {
    //     counter += 1;
    //
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    //
    // println!("The result is {}", result);
    //
    // let mut number = 3;
    //
    // while number != 0 {
    //     println!("{}!", number);
    //
    //     number = number - 1;
    // }
    //
    // println!("LIFTOFF!!!");

    // let a = [10, 20, 30, 40, 50];
    //
    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }

    // for number in (1..4).rev() {
    //     println!("{}!", number);
    // }
    // println!("LIFTOFF!!!");

    let mut f0 = 0;
    let mut f1 = 1;
    let mut n = 10;
    while n >= 0 {
        let f2 = f1 + f0;
        f0 = f1;
        f1 = f2;
        println!("fibo: n={}, f2 = {}", n, f2);
        n -= 1;
    }
}
