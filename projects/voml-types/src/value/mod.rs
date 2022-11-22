use num::FromPrimitive;

use voml_collection::{Decimal, Dict, Integer, List};

mod number;
// mod serializer;

///
pub struct VonSerializer {}

///
#[derive(Debug)]
pub enum Von {
    ///
    Integer(Box<Integer>),
    ///
    Decimal(Box<Decimal>),
    ///
    List(Box<List<Von>>),
    ///
    Dict(Box<Dict<Von>>),
}
