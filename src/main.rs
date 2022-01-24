use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let mut new_transaction = new();
}
#[derive(Debug)]
struct InputTransaction {
    order: String,
    user: i32,
    symbol: String,
    price: i32,
    qty: i32,
    side: String,
    order_user_id: i32,
}

impl InputTransaction{
    fn transaction(&self, data: String) {
        fn parse_csv() {
            let mut rdr = csv::Reader::from_string(data)
                .has_headers(false)
                .delimiter(b';')
                .flexible(true);
            for row in rdr.records() {
                let row = row.unwrap();
                let file = InputTransaction {
                    order: row[0].parse().unwrap(),
                    user: row[1].parse().unwrap(),
                    symbol: row[2].parse().unwrap(),
                    price: row[3].parse().unwrap(),
                    qty: row[4].parse().unwrap(),
                    side: row[5].parse().unwrap(),
                    order_user_id: row[6].parse().unwrap(),
                };  
            }
        }
    }
}

fn new() {
    let mut input = String::new();
    println!("What is the csv");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let path = Path::new(input.trim());
            let display = path.display();
            let mut file = match File::open(&path) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(file) => file,
            };
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("couldn't read {}: {}", display, why),
                Ok(_) => parse_csv(s),
            }
        }
        Err(why) => panic!("couldn't read {}", why),
    }
}

// fn build_transaction(
//     order: String,
//     user: i32,
//     symbol: String,
//     price: i32,
//     qty: i32,
//     side: String,
//     order_user_id: i32,
// ) -> Transaction {
//     Transaction {
//         order: order,
//         user: user,
//         symbol: symbol,
//         price: price,
//         qty: qty,
//         side: side,
//         order_user_id: order_user_id,
//     }
// }
