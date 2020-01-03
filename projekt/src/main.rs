
extern crate csv;
extern crate chrono;
extern crate grabinput;
extern crate linreg;

mod record;
mod knn;
mod reg;

use record::read_records;
use record::Record;

use reg::check_reg_lin;
use std::fs::File;
use std::io::prelude::*;



fn main() {
    let input:Vec<_> = grabinput::by_lines(std::env::args().nth(1)).collect();
    
    let records = read_records(&input);

    let mut file = File::open("in.txt").expect("Cant open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cant cos");
    let test = Record::from_line(&contents);
    
    for i in vec![1,3,5,7,9,11,13]{
        println!("{:?}",knn::knn_classifer(&records, &test, i));
    }
    println!("{:?}",check_reg_lin(&records,"alcohol".to_string()));
    
    
}

