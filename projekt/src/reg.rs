use linreg::{linear_regression, linear_regression_of};
use crate::record::Record;

pub fn check_reg_lin(cos :&Vec<Record>, fieldname: String)->Vec<(f32,f32)>{
    println!("{}",fieldname);
    let mut reg_vector :Vec<f32> = Vec::new();
    if fieldname == "fixed_acid".to_string()
    {
        cos.iter().map(|x| reg_vector.push(x.fixed_acid));
    }
    println!("{:?}",reg_vector);
    vec![(1.0,2.0)]


}