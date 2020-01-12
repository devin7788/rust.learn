extern crate csv;
extern crate rustc_serialize;
use std::thread;
use std::io;
use std::time::{Duration, Instant};

#[derive(RustcDecodable, RustcEncodable)]
struct smallpop {
    city: String,
    region: String,
    country: String,
    population: i32,
}


fn main() {
    let mut rdr =
        csv::Reader::from_path("E:\\rust.learn\\csv\\data\\smallpop.csv").unwrap().has_headers();
    let mut data: Vec<smallpop> = Vec::new();
    for record in rdr.decode() {
        let mut temp: smallpop = record.unwrap();
        println!("len: =>{:?},{},{},{},{}",
                 data.len(),
                 temp.city,
                 temp.region,
                 temp.country,
                 temp.population);
        data.push(temp);
    }
    thread::sleep_ms(500000);

}
