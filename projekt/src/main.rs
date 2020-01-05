
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
    let mut coef = Vec::new();
    for (i,item) in fields.iter().enumerate(){
        println!("{}{}{}{}{:?}",i,",",item,": ",check_reg_lin(&records,item.to_string()));
        coef.push(check_reg_lin(&records,item.to_string()));
    }
    let mut input = String::new();

    println!("{}","Wybierz numer?");

    match io::stdin().read_line(&mut input){
        Ok(_) => {
            let num: i32 = input[..input.len()-1].parse().unwrap();
            let fom : String = match_number(&num); 
            println!("{}{}","Wybrałeś pole: ",fom);
            if fom != "bład"{
                println!("{}{:?}","Klasa jakości: ", knn::select_knn(&records,&test, fom, coef[num as usize].0));
            }
            else
                {println!("{}","Zły input");}
        } 
            ,
        Err(_e) => { println!("{}","Blad"); }

    }   
    
    
}

pub fn match_number(num :&i32)-> String
{
    match num{
       0 => "fixed_acid",
       1 => "volatile_acid",
       2 => "citric_acid",
       3 => "residual_sugar",
       4 => "chlorides",
       5 => "free_sulfur_diox",
       6 => "total_sulfur_diox",
       7 => "density",
       8 => "ph",
       9 => "sulphates",
       10 => "alcohol",
       _ => "bład"     
    }.to_string()
}

