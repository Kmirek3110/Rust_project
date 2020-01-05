
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
use std::io;



fn main() {
    let input:Vec<_> = grabinput::by_lines(std::env::args().nth(1)).collect();
    
    let records = read_records(&input);

    let mut file = File::open("in.txt").expect("Cant open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cant cos");
    let test = Record::from_line(&contents);
    println!("{}","Klasyfikacja po wszystkich polach");
    for i in vec![1,3,5,7,9,11,13]{
        println!("{}{}{}","Po:",i, " sąsiadach");
        println!("{}{:?}","Klasa jakości: ",knn::knn_classifer(&records, &test, i));
    }
    let fields = vec!["fixed_acid","volatile_acid","citric_acid","residual_sugar","chlorides"
    ,"free_sulfur_diox","total_sulfur_diox","density","ph","sulphates","alcohol"];
    println!("{}","Współczynniki do danych atrybutów (regresja liniowa)");
    for item in fields{
        println!("{}{}{:?}",item,": ",check_reg_lin(&records,item.to_string()));
    }
    let mut input = String::new();

    println!("{}","Po czym klasyfikowac podaj jedno pole?");
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("{}{:?}","Klasa jakości: ", knn::select_knn(&records,&test, input));
            },
        Err(_e) => { println!("{}","Blad"); }

    }   
    
    
}

