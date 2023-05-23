use itertools::Itertools;
use pest::iterators::{Pair, Pairs};

mod parser {
    #[derive(pest_derive::Parser)]
    #[grammar = "symono.pest"]
    pub struct SymonoParser;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parsing Error: {0}")]
    Parse(#[from] pest::error::Error<Rule>),
}

pub use parser::Rule;

type Node<'i> = Pair<'i, parser::Rule>;
type Sequence<'i> = Pairs<'i, parser::Rule>;

pub fn parse(input: &str) -> Result<Sequence, Error> {
    use pest::Parser;
    Ok(parser::SymonoParser::parse(Rule::root, input)?
        .next()
        .unwrap()
        .into_child()
        .into_inner())
}

// Transpilers!
pub mod latex;

pub trait Transpile {
    fn latex(self) -> String;
}

impl Transpile for Node<'_> {
    fn latex(self) -> String {
        latex::mono(self)
    }
}

impl Transpile for Sequence<'_> {
    fn latex(self) -> String {
        self.map(|node| node.latex()).join(" ")
    }
}

// Utility
trait PairParent<'i> {
    fn into_child(self) -> Node<'i>;
}

impl<'i> PairParent<'i> for Node<'i> {
    fn into_child(self) -> Node<'i> {
        self.into_inner()
            .next()
            .expect("Parent should have a child")
    }
}
