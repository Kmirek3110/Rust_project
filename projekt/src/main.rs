
extern crate csv;
extern crate chrono;
extern crate grabinput;

mod record;
mod knn;

use record::read_records;
use record::Record;


use std::fs::File;
use std::io::prelude::*;



fn main() {
    let input:Vec<_> = grabinput::by_lines(std::env::args().nth(1)).collect();
    
    let records = read_records(&input);

    let mut file = File::open("in.txt").expect("Cant open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cant cos");
    let mut test = Record::from_line(&contents);
    
    
    println!("{:?}",knn::knn_classifer(records, test, 5));
    
}

