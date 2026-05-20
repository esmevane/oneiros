#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Lens {
    Symbol(Identifier),
    String(StringLiteral),
    Ref(crate::RefToken),
    Integer(IntegerLiteral),
    Predicate(Predicate),
    Union(Box<Lens>, Box<Lens>),
    Intersection(Box<Lens>, Box<Lens>),
    Difference(Box<Lens>, Box<Lens>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Predicate {
    pub(crate) name: PredicateName,
    pub(crate) args: Vec<Lens>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PredicateName(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Identifier(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct StringLiteral(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct IntegerLiteral(i64);

impl IntegerLiteral {
    pub(crate) fn new(value: i64) -> Self {
        Self(value)
    }

    #[allow(dead_code)]
    pub(crate) fn value(self) -> i64 {
        self.0
    }
}

impl From<i64> for IntegerLiteral {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl core::fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl Lens {
    pub(crate) fn symbol(value: impl Into<Identifier>) -> Self {
        Lens::Symbol(value.into())
    }

    pub(crate) fn string(value: impl Into<StringLiteral>) -> Self {
        Lens::String(value.into())
    }

    pub(crate) fn reference(value: impl Into<crate::RefToken>) -> Self {
        Lens::Ref(value.into())
    }

    pub(crate) fn integer(value: impl Into<IntegerLiteral>) -> Self {
        Lens::Integer(value.into())
    }

    pub(crate) fn predicate(
        name: impl Into<PredicateName>,
        args: impl IntoIterator<Item = Lens>,
    ) -> Self {
        Lens::Predicate(Predicate {
            name: name.into(),
            args: args.into_iter().collect(),
        })
    }

    pub(crate) fn union(left: Self, right: Self) -> Self {
        Lens::Union(Box::new(left), Box::new(right))
    }

    pub(crate) fn intersection(left: Self, right: Self) -> Self {
        Lens::Intersection(Box::new(left), Box::new(right))
    }

    pub(crate) fn difference(left: Self, right: Self) -> Self {
        Lens::Difference(Box::new(left), Box::new(right))
    }
}

macro_rules! lens_newtype {
    ($name:ident) => {
        impl $name {
            pub(crate) fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            #[allow(dead_code)]
            pub(crate) fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

lens_newtype!(PredicateName);
lens_newtype!(Identifier);
lens_newtype!(StringLiteral);
