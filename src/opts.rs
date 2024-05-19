
use std:: path::Path;
use clap::Parser;




/*
cargo run -- csv -i test.csv
Opts { cmd: Csv(CsvOpts { input: "test.csv", output: "output.json", delimiter: ',', header: true }) }
*/
//（cli工具）
#[derive(Debug,Parser)]
#[command(name ="rcli", version ,author,about,long_about=None)]
pub struct Opts{
    #[command(subcommand)]
    pub cmd:  Subcommand,
}


//一级命令
#[derive(Debug,Parser)]
pub enum Subcommand {
    #[command(name="csv" ,about="show CSV , or Convert CSV to other formats")]
    Csv(CsvOpts),

}

//一级命令的参数
#[derive(Debug,Parser)]
pub struct CsvOpts{

    #[arg(short,long,value_parser= verify_input_file)] //value_parser 对input的值进行合法性检查
    pub input:String,

    #[arg(short,long,default_value="output.json")]   //default_value -> "".into
    pub output:String,

    #[arg(short,long,default_value_t=',')]
    pub delimiter: char,

    #[arg(long , default_value_t = true)]
    pub header:bool

}


/**
 * 用途： 对csv命令的input参数，进行一个合法性的校验；
 */
fn verify_input_file(filename :&str)->Result<String,&'static str>{
    //判断文件是否存在
    if Path::new(filename).exists() {
         Ok(filename.into())
    }else{
         Err("不存在这个文件")
    }
}
