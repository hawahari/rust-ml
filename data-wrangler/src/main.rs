use std::error::Error;
use std::fs::File;
use std::io::Read;
use csv::Reader;
fn main()->Result<(),Box<dyn Error>>{

    let mut file=File::open("/home/legion/Downloads/new_test_set_with_ruminating (1).txt")?;
    let mut reader=Reader::from_reader(file);
    for result in reader.records(){
    let record=result?;
    println!("{:?}",record);
}
Ok(())
}

