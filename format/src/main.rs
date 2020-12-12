use redis::Commands;

// 这个结构体不能使用 `fmt::Display` 或 `fmt::Debug` 来进行打印。
struct UnPrintable(i32);

// `derive` 属性会自动创建所需的实现，使这个 `struct` 能使用 `fmt::Debug` 打印。
#[derive(Debug)]
struct DebugPrintable(i32);

fn main() {
    println!("Hello, world!");
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // 可以使用命名参数。
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over");
    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number = 1, width = 6);
    // 100前面1个空格, 加起来4个字符宽度
    println!("{number:>4}", number = 100);
    // 你好: 世界
    println!("你好，世界！");
    // println! 会检查使用到的参数数量是否正确。
    // println!("My name is {0}, {1} {0}", "Bond");

    // #[allow(dead_code)]
    // struct Structure(i32);

    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    // println!("This struct `{}` won't print...", Structure(3));
    // 改正 ^ 注释掉此行。
    // connect to redis

    println!("my_key result:{:?}", fetch_an_integer());
}

// https://github.com/mitsuhiko/redis-rs
fn fetch_an_integer() -> String {
    // connect to redis
    let redisAddr = "redis://:admin123@127.0.0.1:6379/1";
    let client = redis::Client::open(redisAddr).unwrap();
    let mut con = client.get_connection().unwrap();
    // throw away the result, just make sure it does not fail
    let _: () = con.set("my_key", 42).unwrap();
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key").unwrap()
}