extern crate parol_runtime;

mod wmla_grammar_grammar;
// The output is version controlled
mod wmla_grammar_grammar_trait;
mod wmla_grammar_parser;

use crate::wmla_grammar_grammar::WmlaGrammarGrammar;
use crate::wmla_grammar_parser::parse;
use anyhow::{anyhow, Context, Result};
use parol_runtime::{log::debug, Report};
use std::{env, fs, time::Instant};

// To generate:
// parol -f ./wmla_grammar.par -e ./wmla_grammar-exp.par -p ./src/wmla_grammar_parser.rs -a ./src/wmla_grammar_grammar_trait.rs -t WmlaGrammarGrammar -m wmla_grammar_grammar -g

struct ErrorReporter;
impl Report for ErrorReporter {}

fn main() -> Result<()> {
    env_logger::init();
    debug!("env logger started");

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let file_name = args[1].clone();
        let input = fs::read_to_string(file_name.clone())
            .with_context(|| format!("Can't read file {}", file_name))?;
        let mut wmla_grammar_grammar = WmlaGrammarGrammar::new();
        let now = Instant::now();
        match parse(&input, &file_name, &mut wmla_grammar_grammar) {
            Ok(_) => {
                let elapsed_time = now.elapsed();
                println!("Parsing took {} milliseconds.", elapsed_time.as_millis());
                if args.len() > 2 && args[2] == "-q" {
                    Ok(())
                } else {
                    println!("Success!\n{}", wmla_grammar_grammar);
                    Ok(())
                }
            }
            Err(e) => ErrorReporter::report_error(&e, file_name),
        }
    } else {
        Err(anyhow!("Please provide a file name as first parameter!"))
    }
}
