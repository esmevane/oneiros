macro_rules! resource_id {
    ($name:ident) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            lorosurgeon::Hydrate,
            lorosurgeon::Reconcile,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema,
        )]
        #[serde(transparent)]
        pub struct $name(pub(crate) crate::Id);

        impl $name {
            pub(crate) fn new() -> Self {
                Self(crate::Id::new())
            }
        }

        impl From<crate::Id> for $name {
            fn from(id: crate::Id) -> Self {
                Self(id)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
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

pub(crate) use resource_id;
