use super::*;

impl Debug for Von {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Von::Boolean(v) => Debug::fmt(v, f),
            Von::Number(v) => Debug::fmt(v, f),
            Von::String(v) => Debug::fmt(v, f),
            Von::Binary(v) => Debug::fmt(v, f),
            Von::Table(v) => Debug::fmt(v, f),
        }
    }
}

impl Von {
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn number<S, N>(name: S, n: N) -> Self
    where
        S: Into<String>,
        N: Into<BigDecimal>,
    {
        Self::Number(Box::new(Number { hint: name.into(), value: n.into() }))
    }

    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn list<S>(name: S, items: List) -> Self
    where
        S: Into<String>,
    {
        Self::Table(Box::new(Table { hint: name.into(), list: items, dict: Default::default() }))
    }

    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn dict<S>(name: S, items: Dict) -> Self
    where
        S: Into<String>,
    {
        Self::Table(Box::new(Table { hint: name.into(), list: Default::default(), dict: items }))
    }
}

impl Von {
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn is_boolean(&self) -> bool {
        matches!(self, Von::Boolean(_))
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn is_integer(&self) -> bool {
        match self {
            Von::Number(v) => v.is_integer(),
            _ => false,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn is_decimal(&self) -> bool {
        match self {
            Von::Number(_) => true,
            _ => false,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn is_string(&self) -> bool {
        matches!(self, Von::String(_))
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn is_binary(&self) -> bool {
        matches!(self, Von::Binary(_))
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn is_list(&self) -> bool {
        match self {
            Von::Table(v) => v.dict.is_empty(),
            _ => false,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn is_dict(&self) -> bool {
        match self {
            Von::Table(v) => v.list.is_empty(),
            _ => false,
        }
    }
}

impl Von {
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn get_integer(&self) -> Option<&Number> {
        match self {
            Von::Number(v) => Some(&**v),
            _ => None,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn mut_integer(&mut self) -> Option<&mut Number> {
        match self {
            Von::Number(v) => Some(&mut **v),
            _ => None,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn get_decimal(&self) -> Option<&Number> {
        match self {
            Von::Number(v) => Some(&**v),
            _ => None,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn mut_decimal(&mut self) -> Option<&mut Number> {
        match self {
            Von::Number(v) => Some(&mut **v),
            _ => None,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn get_list(&self) -> Option<&Table> {
        match self {
            Von::Table(v) => Some(&**v),
            _ => None,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn mut_list(&mut self) -> Option<&mut Vec<Von>> {
        match self {
            Von::Table(v) => Some(&mut v.list),
            _ => None,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn get_dict(&self) -> Option<&Dict> {
        match self {
            Von::Table(v) => Some(&v.dict),
            _ => None,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn mut_dict(&mut self) -> Option<&mut Dict> {
        match self {
            Von::Table(v) => Some(&mut v.dict),
            _ => None,
        }
    }
    // pub fn get_integer(&self) {
    //     match self {
    //         Von::Integer(v) => => Some(& **v),
    //         _ => None,
    //     }
    // }
    // pub fn mut_integer(&mut self) {
    //     match self {
    //         Von::Integer(v) => Some(&mut **v),
    //         _ => None,
    //     }
    // }
}
