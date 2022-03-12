use std::error::Error;
use std::io;
use std::process;
use std::fs;
use std::io::prelude::*;
use std::fs::File;
use serde::Deserialize;
use std::any::{Any, TypeId};
use std::io::Read;

use csv::Reader;

#[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
struct Line {
    id: u8,
    name: String,
    gender: bool,
    f2: f64,
    f3: f64,
    f4: f64,
    empty: Option<String>
}




#[derive(Default)]
struct Table {
    name: String,
    headers: Vec<(String, TypeId)>,
    data: Vec<Vec<Box<dyn Any>>>,
}

impl Table {
    fn add_header(&mut self, header: String, _type: TypeId) {
        self.headers.push((header, _type));
    }

    fn populate_data<R: Read>(
        &mut self,
        rdr: &mut Reader<R>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for record in rdr.records() {
            let record = record?;
            let mut row: Vec<Box<dyn Any>> = vec![];
            for (&(_, type_id), value) in self.headers.iter().zip(record.iter()) {
                if type_id == TypeId::of::<u32>() {
                    row.push(Box::new(value.parse::<u32>()?));
                } else if type_id == TypeId::of::<String>() {
                    row.push(Box::new(value.to_owned()));
                }
            }
            self.data.push(row);
        }
        Ok(())
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Table: {}", self.name)?;
        for (name, _) in self.headers.iter() {
            write!(f, "{}, ", name)?;
        }
        writeln!(f)?;
        for row in self.data.iter() {
            for cell in row.iter() {
                if let Some(&value) = cell.downcast_ref::<u32>() {
                    write!(f, "{}, ", value)?;
                } else if let Some(value) = cell.downcast_ref::<String>() {
                    write!(f, "{}, ", value)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}



pub fn read_csv(){
    let mut table: Table = Default::default();
    table.name = "Foo".to_owned();
    table.add_header("key".to_owned(), TypeId::of::<u32>());
    table.add_header("name".to_owned(), TypeId::of::<String>());
    table.add_header("comment".to_owned(), TypeId::of::<String>());
    let data = "\
key,name,comment
1,foo,foo comment
2,bar,bar comment
";
    let mut rdr = Reader::from_reader(data.as_bytes());
    table.populate_data(&mut rdr).unwrap();
    print!("{}", table);
}


pub fn process_csv(filename: &str) -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    // let mut rdr = csv::Reader::from_reader(io::stdin());
    // let contents = fs::read_to_string("README.md").unwrap();
    
    let f = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(io::BufReader::new(f));
    let header  = rdr.headers()?;
    println!("HEADER:{:?}",header);
    
    
    // for result in rdr.records() {
    //     // The iterator yields Result<StringRecord, Error>, so we check the
    //     // error here.
    //     let record = result?;
    //     println!("{:?}", record);
    // }

    println!("Reading 1 by 1");
    for result in rdr.deserialize() {
        
        let record: Line = result.unwrap();
        // let record: Line = result?;
        
        println!("{:?}", record);
        
        // println!(
        //     "In {}, {} built the {} model. It is a {}.",
        //     record.id,
        //     record.name,
        //     record.gender,
        //     record.f2
        // );
    }

    println!("Finish");

    Ok(())
}

