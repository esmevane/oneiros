macro_rules! domain_id {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub crate::Id);

        impl $name {
            pub fn new() -> Self {
                Self(crate::Id::new())
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl core::str::FromStr for $name {
            type Err = crate::IdParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(s.parse()?))
            }
        }
    };
}

pub(crate) use domain_id;
