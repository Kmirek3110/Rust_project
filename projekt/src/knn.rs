
use crate::record::Record;

use std::collections::BTreeMap;

pub fn knn_classifer(dane: &Vec<Record>, test:&Record, num_of_neigh:i32)->i32{
    
    fn euclidean_distance(test: &Record, elem: &Record)->f32{
        ((test.fixed_acid - elem.fixed_acid).powf(2.0) + 
        (test.volatile_acid - elem.volatile_acid).powf(2.0)  +
        (test.citric_acid - elem.citric_acid).powf(2.0) +
        (test.residual_sugar -elem.residual_sugar).powf(2.0) +
        (test.chlorides - elem.chlorides).powf(2.0)  +
        ((test.free_sulfur_diox - elem.free_sulfur_diox)  as f32).powf(2.0) +
        ((test.total_sulfur_diox - elem.total_sulfur_diox)  as f32).powf(2.0) + 
        (test.density - elem.density).powf(2.0) +
        (test.ph - elem.ph ).powf(2.0) +
        (test.sulphates - elem.sulphates).powf(2.0) +
        (test.alcohol - elem.alcohol).powf(2.0) ).sqrt() as f32
    }

    

    let mut closest_n_dist:Vec<(f32,i32)> = vec![(999.9,0);num_of_neigh as usize];
    
    let mut dana = BTreeMap::new();
    for elem in dane{
       let dist = euclidean_distance(&test, &elem);
       let max = closest_n_dist.iter().max_by(|x,y| x.0.partial_cmp(&y.0).unwrap()).unwrap().0;
       *dana.entry(dist.to_string()).or_insert(0) += 1;
       if dist < max
        {
           closest_n_dist.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
           closest_n_dist[(num_of_neigh - 1) as usize] = (dist, elem.quality);
       }
         
      
    }
    println!("{:?}",closest_n_dist);
    class(&closest_n_dist)
}
pub fn select_knn(dane: &Vec<Record>, test:&Record, pole:String, coef: f64)-> i32{
    
    fn euclidean_distance_s(test: &Record, elem: &Record, pole :&String)->f32{
        
        
        let wynik : f32 ;
        match &pole[..]{
            "fixed_acid" => wynik =   (test.fixed_acid - elem.fixed_acid).powf(2.0),
            "volatile_acid"  => wynik =  (test.volatile_acid - elem.volatile_acid).powf(2.0),
            "citric_acid"  => wynik =  (test.citric_acid - elem.citric_acid).powf(2.0),
            "residual_sugar"  => wynik =  (test.residual_sugar - elem.residual_sugar).powf(2.0),
            "chlorides"  => wynik =  (test.chlorides - elem.chlorides).powf(2.0),
            "free_sulfur_diox"  => wynik =  ((test.free_sulfur_diox - elem.free_sulfur_diox) as f32).powf(2.0),
            "total_sulfur_diox" => wynik =  ((test.total_sulfur_diox - elem.total_sulfur_diox)as f32).powf(2.0),
            "density"  => wynik =  (test.density - elem.density).powf(2.0),
            "ph"  => wynik =   (test.ph - elem.ph).powf(2.0),
            "sulphates" => wynik =  (test.sulphates - elem.sulphates).powf(2.0),
            "alcohol"  => wynik =  (test.alcohol - elem.alcohol).powf(2.0),
            _ => wynik =  0.0,   
       };
       wynik.sqrt() as f32
    }
    
    let mut closest_n_dist:Vec<(f32,i32)> = vec![(999.9,0);7 as usize];
    let mut dana = BTreeMap::new();
    for elem in dane{
        let mut dist = euclidean_distance_s(&test, &elem, &pole);
        dist *= coef as f32;    
        let max = closest_n_dist.iter().max_by(|x,y| x.0.partial_cmp(&y.0).unwrap()).unwrap().0;
        *dana.entry(dist.to_string()).or_insert(0) += 1;
        if dist < max
            {
            closest_n_dist.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            closest_n_dist[(6) as usize] = (dist, elem.quality);
        }
    }
    println!("{:?}",closest_n_dist);
    class(&closest_n_dist)

}


fn class(neigh: &Vec<(f32,i32)> ) -> i32
    {
        let mut classes = BTreeMap::new();
        for neighbour in neigh{
            *classes.entry(neighbour.1).or_insert(0) += 1;
        }
        
        let mut max = -1;
        let mut class = 0;
        for key in classes{
            if key.1 > max { max = key.1; class = key.0;}
        }
        class
    }