fn main() {
    // println!("Hello, world!");
    // let s = String::from("hello, 言则");
    // println!("{}", s);
    //
    // let mut hello = String::from("hello");
    // hello.push_str(", world!"); // push_str() 在字符串后追加字面值
    // println!("{}", hello); // 将打印 `hello, world!`
    //
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("s2 = {}", s2);

    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    s2.push_str(" world!");
    println!("s2 = {}", s2);
}
