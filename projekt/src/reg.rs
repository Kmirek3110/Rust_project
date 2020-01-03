extern crate linreg;
use linreg::{linear_regression, linear_regression_of};
use crate::record::Record;

pub fn check_reg_lin(cos :&Vec<Record>, fieldname: String)->(f64,f64){
    
    let res : (f64,f64);
    if fieldname == "fixed_acid".to_string()
    {
         res = fixed_acid_reg(cos);
    }
    else if fieldname == "volatile_acid".to_string()
    {
        res = volatile_acid_reg(cos);
    }
    else if fieldname == "citric_acid".to_string()
    {
        res = citric_acid_reg(cos);
    }
    else if fieldname == "residual_sugar".to_string()
    {
        res = residual_sugar_reg(cos);
    }
    else if fieldname == "chlorides".to_string()
    {
        res = chlorides_reg(cos);
    }
    else if fieldname == "free_sulfur_diox".to_string()
    {
        res = free_sulfur_diox_reg(cos);
    }
    else if fieldname == "total_sulfur_diox".to_string()
    {
        res = total_sulfur_diox_reg(cos);
    }
    else if fieldname == "density".to_string()
    {
        res = density_reg(cos);
    }
    else if fieldname == "ph".to_string()
    {
        res = ph_reg(cos);
    }
    else if fieldname == "sulphates".to_string()
    {
        res = sulphates_reg(cos);
    }
    else if fieldname == "alcohol".to_string()
    {
        res = alcohol_reg(cos);
    }
    else 
    {
        println!("{}{}", "There is no field like",fieldname);
        res = (0.0,0.0); 
    }
    res
    
}
pub fn fixed_acid_reg(cos :&Vec<Record>)-> (f64,f64){
    let mut reg_vector :Vec<f64> = Vec::new();
    let mut reg_target :Vec<f64> = Vec::new();
    for i in cos {
        reg_vector.push(i.fixed_acid as f64);
        reg_target.push(i.quality as f64);
    }
    linear_regression(&reg_vector,&reg_target).unwrap()
}
pub fn volatile_acid_reg(cos :&Vec<Record>)-> (f64,f64){
    let mut reg_vector :Vec<f64> = Vec::new();
    let mut reg_target :Vec<f64> = Vec::new();
    for i in cos {
        reg_vector.push(i.volatile_acid as f64);
        reg_target.push(i.quality as f64);
    }
    linear_regression(&reg_vector,&reg_target).unwrap()
}
pub fn citric_acid_reg(cos :&Vec<Record>)-> (f64,f64){
    let mut reg_vector :Vec<f64> = Vec::new();
    let mut reg_target :Vec<f64> = Vec::new();
    for i in cos {
        reg_vector.push(i.citric_acid as f64);
        reg_target.push(i.quality as f64);
    }
    linear_regression(&reg_vector,&reg_target).unwrap()
}
pub fn residual_sugar_reg(cos :&Vec<Record>)-> (f64,f64){
    let mut reg_vector :Vec<f64> = Vec::new();
    let mut reg_target :Vec<f64> = Vec::new();
    for i in cos {
        reg_vector.push(i.residual_sugar as f64);
        reg_target.push(i.quality as f64);
    }
    linear_regression(&reg_vector,&reg_target).unwrap()
}
pub fn chlorides_reg(cos :&Vec<Record>)-> (f64,f64){
    let mut reg_vector :Vec<f64> = Vec::new();
    let mut reg_target :Vec<f64> = Vec::new();
    for i in cos {
        reg_vector.push(i.chlorides as f64);
        reg_target.push(i.quality as f64);
    }
    linear_regression(&reg_vector,&reg_target).unwrap()
}
pub fn free_sulfur_diox_reg(cos :&Vec<Record>)-> (f64,f64){
    let mut reg_vector :Vec<f64> = Vec::new();
    let mut reg_target :Vec<f64> = Vec::new();
    for i in cos {
        reg_vector.push(i.free_sulfur_diox as f64);
        reg_target.push(i.quality as f64);
    }
    linear_regression(&reg_vector,&reg_target).unwrap()
}
pub fn total_sulfur_diox_reg(cos :&Vec<Record>)-> (f64,f64){
    let mut reg_vector :Vec<f64> = Vec::new();
    let mut reg_target :Vec<f64> = Vec::new();
    for i in cos {
        reg_vector.push(i.total_sulfur_diox as f64);
        reg_target.push(i.quality as f64);
    }
    linear_regression(&reg_vector,&reg_target).unwrap()
}
pub fn density_reg(cos :&Vec<Record>)-> (f64,f64){
    let mut reg_vector :Vec<f64> = Vec::new();
    let mut reg_target :Vec<f64> = Vec::new();
    for i in cos {
        reg_vector.push(i.density as f64);
        reg_target.push(i.quality as f64);
    }
    linear_regression(&reg_vector,&reg_target).unwrap()
}
pub fn ph_reg(cos :&Vec<Record>)-> (f64,f64){
    let mut reg_vector :Vec<f64> = Vec::new();
    let mut reg_target :Vec<f64> = Vec::new();
    for i in cos {
        reg_vector.push(i.ph as f64);
        reg_target.push(i.quality as f64);
    }
    linear_regression(&reg_vector,&reg_target).unwrap()
}
pub fn sulphates_reg(cos :&Vec<Record>)-> (f64,f64){
    let mut reg_vector :Vec<f64> = Vec::new();
    let mut reg_target :Vec<f64> = Vec::new();
    for i in cos {
        reg_vector.push(i.sulphates as f64);
        reg_target.push(i.quality as f64);
    }
    linear_regression(&reg_vector,&reg_target).unwrap()
}
pub fn alcohol_reg(cos :&Vec<Record>)-> (f64,f64){
    let mut reg_vector :Vec<f64> = Vec::new();
    let mut reg_target :Vec<f64> = Vec::new();
    for i in cos {
        reg_vector.push(i.alcohol as f64);
        reg_target.push(i.quality as f64);
    }
    linear_regression(&reg_vector,&reg_target).unwrap()
}