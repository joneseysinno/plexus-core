use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// TypeSig
// ---------------------------------------------------------------------------

/// A structural type signature used for port compatibility checking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeSig {
    /// Matches any other `TypeSig`. Use as a wildcard input or output.
    Any,
    /// The null / absent type.
    Null,
    /// Boolean.
    Bool,
    /// 64-bit signed integer.
    Int,
    /// 64-bit floating-point number.
    Float,
    /// UTF-8 string.
    String,
    /// Raw byte sequence.
    Bytes,
    /// Homogeneous list of elements with the given element type.
    List(Box<TypeSig>),
    /// Map with string keys and values of the given type.
    Map(Box<TypeSig>),
    /// A user-defined named type (e.g. a domain struct name).
    Named(std::string::String),
}

impl TypeSig {
    /// Returns `true` if a value with type signature `self` can be accepted
    /// where `other` is expected.
    ///
    /// Rules:
    /// - `Any` is compatible with everything (in either position).
    /// - `Named(a)` is compatible with `Named(b)` iff `a == b`.
    /// - `List(a)` is compatible with `List(b)` iff `a` is compatible with `b`.
    /// - `Map(a)` is compatible with `Map(b)` iff `a` is compatible with `b`.
    /// - All other variants require exact equality.
    pub fn is_compatible_with(&self, other: &TypeSig) -> bool {
        match (self, other) {
            (TypeSig::Any, _) | (_, TypeSig::Any) => true,
            (TypeSig::List(a), TypeSig::List(b)) => a.is_compatible_with(b),
            (TypeSig::Map(a), TypeSig::Map(b)) => a.is_compatible_with(b),
            (TypeSig::Named(a), TypeSig::Named(b)) => a == b,
            _ => self == other,
        }
    }
}

impl std::fmt::Display for TypeSig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeSig::Any => write!(f, "Any"),
            TypeSig::Null => write!(f, "Null"),
            TypeSig::Bool => write!(f, "Bool"),
            TypeSig::Int => write!(f, "Int"),
            TypeSig::Float => write!(f, "Float"),
            TypeSig::String => write!(f, "String"),
            TypeSig::Bytes => write!(f, "Bytes"),
            TypeSig::List(inner) => write!(f, "List<{}>", inner),
            TypeSig::Map(inner) => write!(f, "Map<{}>", inner),
            TypeSig::Named(name) => write!(f, "{}", name),
        }
    }
}

// ---------------------------------------------------------------------------
// LayerTag
// ---------------------------------------------------------------------------

/// Semantic layer tag for an atom or block, used for routing and filtering.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayerTag {
    /// Core infrastructure layer.
    Core,
    /// Business domain layer.
    Domain,
    /// External integration layer.
    Integration,
    /// User-defined custom layer.
    Custom(std::string::String),
}

impl std::fmt::Display for LayerTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayerTag::Core => write!(f, "Core"),
            LayerTag::Domain => write!(f, "Domain"),
            LayerTag::Integration => write!(f, "Integration"),
            LayerTag::Custom(s) => write!(f, "Custom({})", s),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_is_compatible_with_everything() {
        assert!(TypeSig::Any.is_compatible_with(&TypeSig::Int));
        assert!(TypeSig::Int.is_compatible_with(&TypeSig::Any));
        assert!(TypeSig::Any.is_compatible_with(&TypeSig::Any));
    }

    #[test]
    fn exact_match_compatible() {
        assert!(TypeSig::Int.is_compatible_with(&TypeSig::Int));
        assert!(TypeSig::Bool.is_compatible_with(&TypeSig::Bool));
    }

    #[test]
    fn mismatched_primitives_incompatible() {
        assert!(!TypeSig::Int.is_compatible_with(&TypeSig::Float));
        assert!(!TypeSig::Bool.is_compatible_with(&TypeSig::String));
    }

    #[test]
    fn named_compatible_same() {
        assert!(TypeSig::Named("Foo".into()).is_compatible_with(&TypeSig::Named("Foo".into())));
    }

    #[test]
    fn named_incompatible_different() {
        assert!(!TypeSig::Named("Foo".into()).is_compatible_with(&TypeSig::Named("Bar".into())));
    }

    #[test]
    fn list_compatibility_recursive() {
        let list_int = TypeSig::List(Box::new(TypeSig::Int));
        let list_any = TypeSig::List(Box::new(TypeSig::Any));
        assert!(list_int.is_compatible_with(&list_any));
        assert!(!list_int.is_compatible_with(&TypeSig::List(Box::new(TypeSig::Bool))));
    }
}
