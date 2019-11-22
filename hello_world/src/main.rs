#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    println!("Hello, world!");
    let x = 5 + /* 90 +  */ 5;
    println!("`x` , x = {}", x);

    println!("{0} this is {1}, {1} this is {0}", "Alice", "Bob");
    println!("{developer}", developer = "LikeRust");

    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    // Pretty print
    println!("{:#?}", peter);
}
// cd rust.learn
// cargo new hello_world
// cargo run

// show:
// hello, world!