
use crate::record::Record;

use std::collections::BTreeMap;

pub fn knn_classifer(dane: &Vec<Record>, test:&Record, num_of_neigh:i32)->i32{
    
    fn euclidean_distance(test: &Record, elem: &Record)->f32{
        ((test.fixed_acid - elem.fixed_acid).powf(2.0) + 
        ((test.volatile_acid - elem.volatile_acid) * 10.0).powf(2.0)  +
        ((test.citric_acid - elem.citric_acid)* 10.0).powf(2.0) +
        (test.residual_sugar -elem.residual_sugar).powf(2.0) +
        ((test.chlorides - elem.chlorides)*100.0).powf(2.0)  +
        (((test.free_sulfur_diox - elem.free_sulfur_diox) / 10) as f32).powf(2.0) +
        (((test.total_sulfur_diox - elem.total_sulfur_diox) / 10) as f32).powf(2.0) + 
        (test.density - elem.density).powf(2.0) +
        (test.ph - elem.ph ).powf(2.0) +
        ((test.sulphates - elem.sulphates)*10.0).powf(2.0) +
        (test.alcohol - elem.alcohol).powf(2.0) ).sqrt() as f32
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