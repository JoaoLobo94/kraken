
// use std::fs;

// fn parse_csv()  {
//     let csv_path = get_csv_path();
//     import_csv(csv_path)
// }

// fn get_csv_path(){
//     println!("Enter csv PATH file to update order book");
//     let mut csv_path = String::new();
//     std::io::stdin()
//         .read_line(&mut csv_path).unwrap();
//     fs::read_to_string(csv_path);
// }

fn import_csv(unparsed_file) {
    let mut rdr = csv::Reader::from_string(unparsed_file)
        .has_headers(false)
        .delimiter(b';')
        .flexible(true);
    for row in rdr.decode() {
        let (order, user, symbol, price, qty, side, order_user_id): (
            String,
            i32,
            String,
            i32,
            i32,
            String,
            i32,
        ) = row.unwrap();
        println!(
            "{}, {}: {} {} {} {} {}",
            order, user, symbol, price, qty, side, order_user_id
        );
    }
}


