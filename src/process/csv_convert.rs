use std::fs;

use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

use crate::cli::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,

    position: String,

    #[serde(rename = "DOB")]
    dob: String,

    nationality: String,

    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone(); // 不加clone会产生借用问题，一个值只能被一个可变引用或多个不可变引用借用，且它们不能同时存在。
    for result in reader.records() {
        let record = result?;
        // headers.iter() -> 使用headers的迭代器
        // record.iter() -> 使用record的迭代器
        // zip() -> 把两个迭代器合并成一个迭代器，每个元素都是一个元组，[(header, record)...]
        // collect::<Value>() -> 将元组迭代器转换成Value类型
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, content)?;

    Ok(())
}
