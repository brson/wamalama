use std::process;

use parol::{build::Builder, ParolErrorReporter};
use parol_runtime::Report;

fn main() {
    // CLI equivalent is:
    // parol -f ./wmla_grammar.par -e ./wmla_grammar-exp.par -p ./src/wmla_grammar_parser.rs -a ./src/wmla_grammar_grammar_trait.rs -t WmlaGrammarGrammar -m wmla_grammar_grammar -g
    if let Err(err) = Builder::with_explicit_output_dir("src")
        .grammar_file("wmla_grammar.par")
        .expanded_grammar_output_file("../wmla_grammar-exp.par")
        .parser_output_file("wmla_grammar_parser.rs")
        .actions_output_file("wmla_grammar_grammar_trait.rs")
        .enable_auto_generation()
        .user_type_name("WmlaGrammarGrammar")
        .user_trait_module_name("wmla_grammar_grammar")
        .trim_parse_tree()
        .generate_parser()
    {
        ParolErrorReporter::report_error(&err, "wmla_grammar.par").unwrap_or_default();
        process::exit(1);
    }
}
