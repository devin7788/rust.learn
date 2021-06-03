fn main() {
    println!("Hello, world!");
    let user = User {
        username: String::from("somewhere123"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("username:{}", user.username);

    let user2 = build_user(String::from("123"), String::from("456"));
    println!("username:{}", user2.username);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user.active,
        sign_in_count: user.sign_in_count,
    };
    println!("username:{}", user2.username);

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user
    };
    println!("active:{:#?}", user3);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    println!("area= {}", area(15, 10));

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:#?}", rect1);
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}