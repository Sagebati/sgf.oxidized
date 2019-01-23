extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;
use pest::iterators::Pair;
use std::fs;
use pest::Parser;


#[derive(Parser)]
#[grammar = "grammar/sgf.pest"]
pub struct SgfParser;

///
/// A property can have multiple values like AW[][][]
///
#[derive(Clone, Debug)]
pub struct Property {
    pub ident: String,
    pub values: Vec<String>,
}

impl Property {
    ///
    /// Needs the Pair with the rule property
    ///
    fn from_pair_pest(pair: Pair<Rule>) -> Property {
        let inner = pair.into_inner();
        let mut ident = "".to_string();
        let mut values = Vec::<String>::new();
        for i in inner {
            match i.as_rule() {
                Rule::prop_ident => {
                    ident = i.as_str().to_string();
                }

                Rule::prop_value => {
                    values.push(i.as_str().to_string());
                }

                _ => unreachable!()
            }
        }

        Property { ident, values }
    }
}

///
/// A node is a group of properties indexed by the Id
/// ;XX[FOO]AZ[BAR]FR[FOO]
///
pub type Node = HashMap<String, Property>;


///
/// Sequence of nodes.
/// ;XX[FOO]AZ[BAR]FR[FOO];XX[FOO]
///
pub type NodeSeq = Vec<Node>;


#[derive(Clone, Debug)]
pub struct GameTree {
    /// Contains the information of the game
    pub root: Node,
    /// Contains the game plays
    pub seq: NodeSeq,

}

impl GameTree {
    pub fn from_pair(pair: Pair<Rule>) -> GameTree {
        let iner = pair.into_inner();
        let mut root = HashMap::new();
        let mut seq = Vec::new();
        for p in iner {
            match p.as_rule() {
                Rule::node => root = parse_node(p),
                Rule::node_seq => seq = parse_nodeseq(p),
                _ => unimplemented!("The tail is not implemented for the moment"),
            }
        }

        GameTree { root, seq }
    }
}

///
    /// Needs a pair with the node rule
    ///
pub fn parse_node(pair: Pair<Rule>) -> Node {
    let mut new = Node::new();
    pair.into_inner().into_iter()
        .for_each(|property| {
            let pop = Property::from_pair_pest(property);
            new.insert(pop.ident.clone(), pop);
        });
    new
}

///
/// Needs a pair of rule node seq
///
fn parse_nodeseq(pair: Pair<Rule>) -> NodeSeq {
    pair.into_inner().into_iter()
        .map(|node| parse_node(node))
        .collect()
}

///
/// Needs a pair of rule collection
///
pub fn parse_collection(pair: Pair<Rule>) -> Collection {
    pair.into_inner().into_iter()
        .map(|gametree| GameTree::from_pair(gametree))
        .collect()
}

pub type Collection = Vec<GameTree>;


#[derive(Clone, Debug)]
pub struct Sgf {
    pub collection: Collection
}

impl Sgf {
    pub fn from_str(lines: &str) -> Result<Sgf, String> {
        if let Ok(mut x) = SgfParser::parse(Rule::file, lines) {
            Ok(Sgf::from_pair(x.next().unwrap()))
        } else {
            Err("Parsing error".to_string())
        }
    }

    pub fn from_file(path: &str) -> Result<Sgf, String> {
        if let Ok(lines) = fs::read_to_string(path) {
            Sgf::from_str(&lines)
        } else {
            Err("Reading the file error.".to_string())
        }
    }

    pub fn from_pair(pair: Pair<Rule>) -> Sgf {
        Sgf { collection: parse_collection(pair) }
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use std::fs;
    use crate::*;


    #[test]
    fn parser() {
        let unparsed_file = fs::read_to_string("test.sgf").expect("cannot readfile");
        let collection = SgfParser::parse(Rule::file, &unparsed_file)
            .expect("unsuccessful parse") // unwrap the parse result
            .next().unwrap(); // get and unwrap the `file` rule; never fails
        dbg!(Sgf::from_pair(collection));
    }
}
