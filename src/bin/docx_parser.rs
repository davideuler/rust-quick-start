// docx file parser, convert word file to text.
// [dependencies]
//docx-rs = "0.4.6"
//clap = { version = "4.2.2", features = ["derive"] }
//serde_json = "1.0.96"
//anyhow = "1.0.70"


use clap::Parser;
use docx_rs::*;
use serde_json::Value;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

pub fn parse_docx(file_name: &str, output_text: & mut String) -> anyhow::Result<()> {
    parse_docx_from_bytes(&read_to_vec(file_name)?, output_text)?;
    Ok(())
}

pub fn parse_docx_from_bytes(data: &[u8],output_text: & mut String) -> anyhow::Result<()> {
    let data: Value = serde_json::from_str(&read_docx(data)?.json())?;
    if let Some(children) = data["document"]["children"].as_array() {
        children.iter().for_each(|node: &Value| {
            // println!("type: {}", node["type"]);
            // println!("node: {}", node);
            read_children(node, output_text);
            output_text.push_str("\n");
        })
    }
    Ok(())
}

fn read_children(node: &Value, output_text: & mut String) {
    if let Some(children) = node["data"]["children"].as_array() {
        children.iter().for_each(|child| {
            if child["type"] != "text" {
                read_children(child, output_text);
            } else {
                // println!("text: {}", child["data"]["text"]);
                output_text.push_str(child["data"]["text"].as_str().unwrap());
                // output_text.push_str("\n");
            }
        });
    }
}

fn read_to_vec(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    std::fs::File::open(file_name)?.read_to_end(&mut buf)?;
    Ok(buf)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut output_text = String::new();

    parse_docx(&args.name, & mut output_text)?;
    println!("{}", output_text);
    Ok(())
}
