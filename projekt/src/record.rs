#[derive(Debug)]
pub struct Record {
    pub fixed_acid: f32,
    pub volatile_acid: f32,
    pub citric_acid: f32,
    pub residual_sugar: f32,
    pub chlorides: f32,
    pub free_sulfur_diox: i32,
    pub total_sulfur_diox: i32,
    pub density: f32,
    pub ph: f32,
    pub sulphates: f32,
    pub alcohol: f32,
    pub quality: i32,

}
impl Record{
    pub fn from_line(line: &str)->Record
    {
        let mut data = line.split(',');
        
        Record{
            fixed_acid: data.next().and_then(|n| n.trim().replace("\"","").parse::<f32>().ok()).unwrap_or_default().to_owned(),
            volatile_acid: data.next().and_then(|n| n.trim().replace("\"","").parse::<f32>().ok()).unwrap_or_default().to_owned()*10.0,
            citric_acid: data.next().and_then(|n| n.trim().replace("\"","").parse::<f32>().ok()).unwrap_or_default().to_owned()*10.0,
            residual_sugar: data.next().and_then(|n| n.trim().replace("\"","").parse::<f32>().ok()).unwrap_or_default().to_owned(),
            chlorides: data.next().and_then(|n| n.trim().replace("\"","").parse::<f32>().ok()).unwrap_or_default().to_owned()*100.0,
            free_sulfur_diox: data.next().and_then(|n| n.trim().replace("\"","").parse::<i32>().ok()).unwrap_or_default().to_owned()/10,
            total_sulfur_diox: data.next().and_then(|n| n.trim().replace("\"","").parse::<i32>().ok()).unwrap_or_default().to_owned()/10,
            density: data.next().and_then(|n| n.trim().replace("\"","").parse::<f32>().ok()).unwrap_or_default().to_owned()*10.0,
            ph: data.next().and_then(|n| n.trim().replace("\"","").parse::<f32>().ok()).unwrap_or_default().to_owned(),
            sulphates: data.next().and_then(|n| n.trim().replace("\"","").parse::<f32>().ok()).unwrap_or_default().to_owned()*10.0,
            alcohol: data.next().and_then(|n| n.trim().replace("\"","").parse::<f32>().ok()).unwrap_or_default().to_owned(),
            quality: data.next().and_then(|n| n.trim().replace("\"","").parse::<i32>().ok()).unwrap_or_default().to_owned(),

        }
    }
}

pub fn read_records(input: &[String]) -> Vec<Record>{

    let mut records = Vec::with_capacity(input.len());
    for line in input{
        records.push(Record::from_line(line));
        
    }
    records
}
