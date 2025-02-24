use std::{fmt, str::FromStr};

use clap::Parser;

use super::verify_file;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[clap(short, long, value_parser = verify_file)] // 校验输入文件
    pub input: String,

    #[clap(short, long)] // "output.json".into()
    pub output: Option<String>,

    #[clap(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[clap(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[clap(long, default_value_t = false)]
    pub header: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
