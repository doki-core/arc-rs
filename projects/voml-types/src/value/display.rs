use super::*;

impl Debug for Von {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Von::Boolean(v) => Debug::fmt(v, f),
            Von::Number(v) => Debug::fmt(v, f),
            Von::String(v) => Debug::fmt(v, f),
            Von::Binary(v) => Debug::fmt(v, f),
            Von::Table(v) => Debug::fmt(v, f),
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
        Self::Table(Box::new(List { hint: name.into(), list: items }))
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
        matches!(self, Von::Table(_))
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
        matches!(self, Von::Dict(_))
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
    pub fn get_list(&self) -> Option<&Table<Von>> {
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
    pub fn mut_list(&mut self) -> Option<&mut List> {
        match self {
            Von::Table(v) => Some(&mut **v),
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
