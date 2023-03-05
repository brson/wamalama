use crate::wmla_grammar_grammar_trait::{WmlaGrammar, WmlaGrammarGrammarTrait};
#[allow(unused_imports)]
use parol_runtime::Result;
use std::fmt::{Debug, Display, Error, Formatter};

///
/// Data structure that implements the semantic actions for our WmlaGrammar grammar
/// !Change this type as needed!
///
#[derive(Debug, Default)]
pub struct WmlaGrammarGrammar<'t> {
    pub wmla_grammar: Option<WmlaGrammar<'t>>,
}

impl WmlaGrammarGrammar<'_> {
    pub fn new() -> Self {
        WmlaGrammarGrammar::default()
    }
}

impl Display for WmlaGrammar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl Display for WmlaGrammarGrammar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match &self.wmla_grammar {
            Some(wmla_grammar) => writeln!(f, "{}", wmla_grammar),
            None => write!(f, "No parse result"),
        }
    }
}

impl<'t> WmlaGrammarGrammarTrait<'t> for WmlaGrammarGrammar<'t> {
    // !Adjust your implementation as needed!

    /// Semantic action for non-terminal 'WmlaGrammar'
    fn wmla_grammar(&mut self, arg: &WmlaGrammar<'t>) -> Result<()> {
        self.wmla_grammar = Some(arg.clone());
        Ok(())
    }
}
