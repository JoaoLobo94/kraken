// mod file_parser;
// use std::io;
// use std::io::prelude::*;

// fn main() {
//     let file_parser = new();
//     println!("{}", file_parser)
// }
// struct Parser {
//     order: String,
//     user: i32,
//     symbol: String,
//     price: i32,
//     qty: i32,
//     side: String,
//     order_user_id: i32,
// }

// // struct FileReader{
// //     path: String
// // }

// // impl FileReader{
//     fn new() -> String{
//         println!("OK: aaa !");
//         let reader = io::stdin();
//         let mut buffer: String = String::new();

//         reader.read_line(&mut buffer)
//             .ok()
//             .expect("ERRMSG");
//         // let mut rdr = csv::Reader::from_string(buffer);
//         return buffer;

//     }

//     // fn (path: String) -> Self {
//     //     Self{
//     //        path
//     //     }
//     // }
// // }

// impl Parser {
//     fn new(
//         order: String,
//         user: i32,
//         symbol: String,
//         price: i32,
//         qty: i32,
//         side: String,
//         order_user_id: i32,
//     ) -> Self {
//         Self {
//             order,
//             user,
//             symbol,
//             price,
//             qty,
//             side,
//             order_user_id,
//         }
//     }

//     fn run(self) {
//         println!("parsing... {}", self.order)
//     }
// }
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
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

fn parse_csv(data: String) {
    let mut rdr = csv::Reader::from_string(data)
        .has_headers(false)
        .delimiter(b';')
        .flexible(true);
    for row in rdr.records() {
        let row = row.unwrap();
        println!("{:?}", row);
    }
}
