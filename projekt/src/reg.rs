extern crate linreg;
use linreg::{linear_regression, linear_regression_of};
use crate::record::Record;

pub fn check_reg_lin(cos :&Vec<Record>, fieldname: String)->Vec<(f32,f32)>{
    println!("{}",fieldname);
    let mut reg_vector :Vec<f64> = Vec::new();
    let mut reg_target :Vec<f64> = Vec::new();
    if fieldname == "fixed_acid".to_string()
    {
        println!("{}","weszło");
        for i in cos {
            reg_vector.push(i.fixed_acid as f64);
            reg_target.push(i.quality as f64);
        }
        
    }
    let res: (f64,f64) = linear_regression(&reg_vector,&reg_target).unwrap();
    println!("{:?}",res);
    vec![(1.0,2.0)]


}