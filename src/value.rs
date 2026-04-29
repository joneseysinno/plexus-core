use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::types::TypeSig;

// ---------------------------------------------------------------------------
// Value
// ---------------------------------------------------------------------------

/// A runtime-typed value that can flow through ports in an frp graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    /// The absence of a value.
    Null,
    /// A boolean value.
    Bool(bool),
    /// A 64-bit signed integer.
    Int(i64),
    /// A 64-bit floating-point number.
    Float(f64),
    /// A UTF-8 string.
    Str(std::string::String),
    /// Raw bytes.
    Bytes(Vec<u8>),
    /// An ordered, homogeneous-or-heterogeneous list.
    List(Vec<Value>),
    /// A string-keyed map of values.
    Map(BTreeMap<std::string::String, Value>),
}

impl Value {
    /// Return the [`TypeSig`] that describes this value's runtime type.
    pub fn type_sig(&self) -> TypeSig {
        match self {
            Value::Null => TypeSig::Null,
            Value::Bool(_) => TypeSig::Bool,
            Value::Int(_) => TypeSig::Int,
            Value::Float(_) => TypeSig::Float,
            Value::Str(_) => TypeSig::String,
            Value::Bytes(_) => TypeSig::Bytes,
            Value::List(items) => {
                // Infer element type from first item; fall back to Any for empty lists.
                let elem_sig = items.first().map(|v| v.type_sig()).unwrap_or(TypeSig::Any);
                TypeSig::List(Box::new(elem_sig))
            }
            Value::Map(map) => {
                let val_sig = map
                    .values()
                    .next()
                    .map(|v| v.type_sig())
                    .unwrap_or(TypeSig::Any);
                TypeSig::Map(Box::new(val_sig))
            }
        }
    }

    /// Returns `true` if this value is `Value::Null`.
    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Attempt to get the inner `bool`.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Attempt to get the inner `i64`.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Attempt to get the inner `f64`.
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Attempt to get the inner string slice.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Attempt to get a reference to the inner list.
    pub fn as_list(&self) -> Option<&Vec<Value>> {
        match self {
            Value::List(v) => Some(v),
            _ => None,
        }
    }

    /// Attempt to get a reference to the inner map.
    pub fn as_map(&self) -> Option<&BTreeMap<std::string::String, Value>> {
        match self {
            Value::Map(m) => Some(m),
            _ => None,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Str(s) => write!(f, "\"{}\"", s),
            Value::Bytes(b) => write!(f, "<bytes:{}>", b.len()),
            Value::List(v) => write!(f, "[..{}]", v.len()),
            Value::Map(m) => write!(f, "{{..{}}}", m.len()),
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl From<std::string::String> for Value {
    fn from(s: std::string::String) -> Self {
        Value::Str(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Str(s.to_owned())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_sig_matches_variant() {
        assert_eq!(Value::Null.type_sig(), TypeSig::Null);
        assert_eq!(Value::Bool(true).type_sig(), TypeSig::Bool);
        assert_eq!(Value::Int(1).type_sig(), TypeSig::Int);
        assert_eq!(Value::Float(1.0).type_sig(), TypeSig::Float);
        assert_eq!(Value::Str("hi".into()).type_sig(), TypeSig::String);
    }

    #[test]
    fn list_type_sig_infers_element() {
        let v = Value::List(vec![Value::Int(1), Value::Int(2)]);
        assert_eq!(v.type_sig(), TypeSig::List(Box::new(TypeSig::Int)));
    }

    #[test]
    fn empty_list_type_sig_is_any() {
        let v = Value::List(vec![]);
        assert_eq!(v.type_sig(), TypeSig::List(Box::new(TypeSig::Any)));
    }

    #[test]
    fn from_conversions() {
        assert_eq!(Value::from(42i64), Value::Int(42));
        assert_eq!(Value::from("hello"), Value::Str("hello".into()));
    }
}
