extern crate quick_csv;

fn main() {
    let data = "a,b\r\nc,d\r\ne,f";
    let csv = quick_csv::Csv::from_string(data);
    for row in csv.into_iter() {
        // work on csv row ...
        if let Ok(_) = row {
            println!("new row!");
        } else {
            println!("cannot read next line");
        }
    }
}
