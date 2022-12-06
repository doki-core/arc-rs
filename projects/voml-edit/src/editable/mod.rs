use std::convert::TryInto;

use serde::{Deserialize, Serialize};

use crate::VQuery;

/// Configuration of [`VomlModel`]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VomlEditor {}

pub struct VomlModel {}

impl VomlModel {
    pub fn get_value<Q, V>(&self, query: Q) -> Option<&V>
    where
        Q: TryInto<VQuery>,
    {
        None
    }
    pub fn mut_value<Q, V>(&mut self, query: Q) -> Option<&mut V>
    where
        Q: TryInto<VQuery>,
    {
        None
    }
    pub fn set_value<Q, V>(&mut self, query: Q, value: V) -> Option<V>
    where
        Q: TryInto<VQuery>,
    {
        None
    }
}
