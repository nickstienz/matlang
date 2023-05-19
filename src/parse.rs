use pest::{error::Error, iterators::Pairs};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MatlangParser;

pub fn parse(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    unimplemented!()
}
