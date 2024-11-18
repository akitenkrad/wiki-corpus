use std::process;

use parol::{build::Builder, ParolErrorReporter};
use parol_runtime::Report;

fn main() {
    // CLI equivalent is:
    // parol -f ./wiki_grammar.par -e ./wiki_grammar-exp.par -p ./src/wiki_grammar_parser.rs -a ./src/wiki_grammar_grammar_trait.rs -t WikiGrammarGrammar -m wiki_grammar_grammar -g
    if let Err(err) = Builder::with_explicit_output_dir("src")
        .grammar_file("wiki.par")
        .expanded_grammar_output_file("../wiki-exp.par")
        .parser_output_file("wiki_parser.rs")
        .actions_output_file("wiki_grammar_trait.rs")
        .enable_auto_generation()
        .user_type_name("WikiGrammar")
        .user_trait_module_name("wiki_grammar")
        .trim_parse_tree()
        .generate_parser()
    {
        ParolErrorReporter::report_error(&err, "wiki.par").unwrap_or_default();
        process::exit(1);
    }
}
