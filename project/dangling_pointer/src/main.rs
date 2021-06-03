fn main() {
    let reference_to_nothing = no_dangle();
    println!("reference_to_nothing={}", reference_to_nothing);
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}