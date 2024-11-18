extern crate parol_runtime;

mod wiki_grammar;
// The output is version controlled
mod wiki_grammar_trait;
mod wiki_parser;

use crate::wiki_grammar::WikiGrammar;
use crate::wiki_parser::parse;
use anyhow::{Error, Result};
use wiki_grammar_trait::ValueListGroup;

use parol_runtime::Report;

#[cfg(test)]
mod tests;

struct ErrorReporter;
impl Report for ErrorReporter {}

pub fn analyze(input: String) -> Result<String> {
    let mut wiki_grammar_grammar = WikiGrammar::new();
    let mut result: Vec<String> = Vec::new();
    let file_name = "";
    match parse(&input, &file_name, &mut wiki_grammar_grammar) {
        Ok(_) => {
            let vg = wiki_grammar_grammar.wiki_grammar.unwrap().value.value_list;
            for v in vg {
                let v = v.value_list_group.as_ref();
                match v {
                    ValueListGroup::String(_v) => {
                        result.push(_v.string.string.text().to_string());
                        result.push(" ".to_string());
                    }
                    ValueListGroup::Link(_v) => {
                        result.push(_v.link.string.string.text().to_string());
                        result.push(" ".to_string());
                    }
                    ValueListGroup::Attribute(_v) => {}
                    ValueListGroup::Attribute2(_v) => {}
                }
            }
        }
        Err(e) => {
            let _ = ErrorReporter::report_error(&e, file_name);
            println!("INPUT: {}", input);
            return Err(Error::msg("Error parsing input"));
        }
    }
    let text = result.join(" ");
    let ptn_ws = regex::Regex::new(r"\s\s+").unwrap();
    let text = ptn_ws.replace_all(&text, " ").to_string();
    Ok(text)
}
