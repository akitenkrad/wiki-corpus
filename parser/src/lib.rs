use anyhow::Result;
use bzip2::read::{BzEncoder, MultiBzDecoder};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub timestamp: String,
    pub texts: Vec<String>,
}

pub fn parse_xml(input_filepath: String) -> Result<()> {
    let decoder = MultiBzDecoder::new(File::open(input_filepath.clone()).unwrap());
    let filesize = File::open(input_filepath.clone())
        .unwrap()
        .metadata()
        .unwrap()
        .len();
    let mut reader = Reader::from_reader(BufReader::new(decoder));
    let mut writer = BufWriter::new(
        File::create(Path::new(&input_filepath.replace(".xml", "")).with_extension("jsonl.bz2"))
            .unwrap(),
    );
    let pbar = indicatif::ProgressBar::new(filesize);
    pbar.set_style(indicatif::ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
    ).unwrap().progress_chars("=>-"));

    let mut buf = Vec::new();
    let mut is_tilte = false;
    let mut is_timestamp = false;
    let mut is_text = false;
    let ptn_comment = regex::Regex::new(r"<!--.*?-->").unwrap();
    let ptn_ref = regex::Regex::new(r"<ref.*?>.*?</ref>").unwrap();
    let ptn_sub = regex::Regex::new(r"<sub>.*?</sub>").unwrap();
    let ptn_math = regex::Regex::new(r"<math>.*?</math>").unwrap();
    let ptn_dash = regex::Regex::new(r"==+.*?==+").unwrap();
    let ptn_quote = regex::Regex::new(r"''+(?<TEXT>.*?)''+").unwrap();

    let mut title = String::new();
    let mut timestamp = String::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                pbar.inc(e.name().as_ref().len() as u64);
                if e.name().as_ref() == b"text" {
                    is_text = true;
                } else if e.name().as_ref() == b"title" {
                    is_tilte = true;
                } else if e.name().as_ref() == b"timestamp" {
                    is_timestamp = true;
                }
            }
            Ok(Event::Text(e)) => {
                pbar.inc(e.unescape().unwrap().len() as u64);
                if is_text {
                    let text = e.unescape().unwrap().to_string();
                    let text = text.replace("&quot;", "'");
                    let text = text.replace("&lt;", "<");
                    let text = text.replace("&gt;", ">");
                    if text.starts_with("#REDIRECT") {
                        continue;
                    }

                    let mut cleaned_text = String::new();
                    for line in text.split("\n") {
                        let line = line.trim();
                        if line.starts_with("[[File:") && line.ends_with("]]") {
                            continue;
                        } else if line.starts_with("*") {
                            continue;
                        }
                        cleaned_text.push_str(line);
                    }

                    match wiki_parser::analyze(cleaned_text) {
                        Ok(cleaned_text) => {
                            if cleaned_text.is_empty() {
                                continue;
                            } else {
                                let cleaned_text = ptn_comment.replace_all(&cleaned_text, "");
                                let cleaned_text = ptn_ref.replace_all(&cleaned_text, "");
                                let cleaned_text = ptn_sub.replace_all(&cleaned_text, "");
                                let cleaned_text = ptn_math.replace_all(&cleaned_text, "");
                                let cleaned_text = ptn_dash.replace_all(&cleaned_text, "");
                                let cleaned_text = ptn_quote.replace_all(&cleaned_text, "$TEXT");

                                let mut lines: Vec<String> = Vec::new();
                                for l in cleaned_text.split("\n") {
                                    let l = l.trim();
                                    if l.is_empty() {
                                        continue;
                                    }
                                    let _lines: Vec<String> = l
                                        .split(". ")
                                        .map(|s| format!("{}.", s.trim()))
                                        .filter(|s| s.chars().count() >= 3)
                                        .collect();
                                    lines.extend(_lines);
                                }
                                let article = Article {
                                    title: title.clone(),
                                    timestamp: timestamp.clone(),
                                    texts: lines.clone(),
                                };

                                let mut json_text = serde_json::to_string(&article).unwrap();
                                json_text.push('\n');
                                let compressor = BzEncoder::new(
                                    json_text.as_bytes(),
                                    bzip2::Compression::best(),
                                );
                                let compressed =
                                    compressor.bytes().collect::<Result<Vec<u8>, _>>().unwrap();
                                writer.write(&compressed).unwrap();
                            }
                        }
                        Err(e) => {
                            println!("[Error]: {}", e);
                            continue;
                        }
                    }
                }
                if is_tilte {
                    title = e.unescape().unwrap().to_string();
                    is_tilte = false;
                }
                if is_timestamp {
                    timestamp = e.unescape().unwrap().to_string();
                    is_timestamp = false;
                }
            }
            Ok(Event::End(ref e)) => {
                pbar.inc(e.name().as_ref().len() as u64);
                if e.name().as_ref() == b"text" {
                    is_text = false;
                }
                if e.name().as_ref() == b"title" {
                    is_tilte = false;
                }
                if e.name().as_ref() == b"timestamp" {
                    is_timestamp = false;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => (),
        }
        buf.clear();
        writer.flush().unwrap();
    }
    Ok(())
}
