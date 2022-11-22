use super::*;

impl Debug for Von {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Von::Boolean(v) => Debug::fmt(v, f),
            Von::Integer(v) => Debug::fmt(v, f),
            Von::Number(v) => Debug::fmt(v, f),
            Von::String(v) => Debug::fmt(v, f),
            Von::Binary(v) => Debug::fmt(v, f),
            Von::List(v) => Debug::fmt(v, f),
            Von::Dict(v) => Debug::fmt(v, f),
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
    pub fn list<S>(name: S, items: Vec<Von>) -> Self
    where
        S: Into<String>,
    {
        Self::List(Box::new(List { hint: name.into(), list: items }))
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
        matches!(self, Von::Integer(_))
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
        matches!(self, Von::Integer(_))
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
        matches!(self, Von::Integer(_))
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
        matches!(self, Von::Integer(_))
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
        matches!(self, Von::Integer(_))
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
        matches!(self, Von::Integer(_))
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
    pub fn get_list(&self) -> Option<&Integer> {
        match self {
            Von::Integer(v) => Some(&**v),
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
    pub fn mut_list(&mut self) -> Option<&mut Integer> {
        match self {
            Von::Integer(v) => Some(&mut **v),
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
    pub fn get_dict(&self) -> Option<&Dict<Von>> {
        match self {
            Von::Dict(v) => Some(&**v),
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
    pub fn mut_dict(&mut self) -> Option<&mut Dict<Von>> {
        match self {
            Von::Dict(v) => Some(&mut **v),
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
