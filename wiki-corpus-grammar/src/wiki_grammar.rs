use crate::wiki_grammar_trait::{Wiki, WikiGrammarTrait};
#[allow(unused_imports)]
use parol_runtime::{Result, Token};
use std::fmt::{Debug, Display, Error, Formatter};

///
/// Data structure that implements the semantic actions for our WikiGrammar grammar
/// !Change this type as needed!
///
#[derive(Debug, Default)]
pub struct WikiGrammar<'t> {
    pub wiki_grammar: Option<Wiki<'t>>,
}

impl WikiGrammar<'_> {
    pub fn new() -> Self {
        WikiGrammar::default()
    }
}

impl Display for Wiki<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl Display for WikiGrammar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match &self.wiki_grammar {
            Some(wiki_grammar) => writeln!(f, "{}", wiki_grammar),
            None => write!(f, "No parse result"),
        }
    }
}

impl<'t> WikiGrammarTrait<'t> for WikiGrammar<'t> {
    // !Adjust your implementation as needed!

    /// Semantic action for non-terminal 'WikiGrammar'
    fn wiki(&mut self, arg: &Wiki<'t>) -> Result<()> {
        self.wiki_grammar = Some(arg.clone());
        Ok(())
    }
}
