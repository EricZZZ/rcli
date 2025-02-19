use std::path::Path;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli",version,author,about,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[clap(short, long, value_parser = verify_input_file)] // 校验输入文件
    pub input: String,
    #[clap(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,
    #[clap(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[clap(long, default_value_t = false)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}
