use num::FromPrimitive;

use voml_collection::{Decimal, Dict, Integer, List};

mod number;
// mod serializer;
use num::ToPrimitive;
use std::fmt::{Debug, Formatter};

///
pub struct VonSerializer {}

///
pub enum Von {
    Boolean(bool),
    ///
    Integer(Box<Integer>),
    ///
    Decimal(Box<Decimal>),
    ///
    List(Box<List<Von>>),
    ///
    Dict(Box<Dict<Von>>),
}

mod display;
