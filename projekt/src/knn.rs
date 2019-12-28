
use crate::record::Record;

pub fn knn_classifer(Dane: Vec<Record>, test:Record, num_of_neigh:i32)->i32{
    
    fn euclidean_distance(elem: Record)->f32{
        unimplemented!();
    }
    let mut closest_n_dist:Vec<(f32,i32)> = vec![(999.9,0);num_of_neigh as usize];
    println!("{:?}",closest_n_dist);
    //for elem in Dane{
     //   let dist = euclidean_distance(elem);
      //  if dist < closest_n_dist.iter().max_by(|x,y| x.1.cmp(y.1)).unwrap()
       // {
        //    closest_n_dist.sort_by_key(|k| k.1);
         //   closest_n_dist[num_of_neigh] = (dist, elem.quality);
     //  }
    //}
    
    3
}