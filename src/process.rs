

use std::fs;
use csv::Reader;
use serde::{Serialize,Deserialize};



#[derive(Debug,Deserialize,Serialize)]
pub struct Player{
    #[serde(rename ="Name")]
    pub name :String,
    #[serde(rename ="Position")]
    pub position:String,
    #[serde(rename ="DOB")]
    pub dob:String,
    #[serde(rename ="Nationality")]
    pub nationality:String,
    #[serde(rename ="Kit Number")]
    pub kit:u8,


}

pub fn process_csv(input :&str,output :&str) -> anyhow::Result<()>{
    let mut reader =  Reader::from_path(input)?;
    let mut ret =Vec::with_capacity(128);
    for result in reader.deserialize(){
      //Player { name: "Rodrigo Bentancur", position: "Central Midfield", dob: "Jun 25, 1997 (22)", nationality: "Uruguay", kit: 30 }
      let record :Player = result?;
      ret.push(record);
    }
    let json =serde_json::to_string_pretty(&ret)?;
    fs::write(output,json)?;
    Ok(())
}
