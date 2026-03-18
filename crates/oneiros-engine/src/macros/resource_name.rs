macro_rules! resource_name {
    ($name:ident) => {
        #[derive(
            Clone,
            Debug,
            PartialEq,
            Eq,
            Hash,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema,
        )]
        #[serde(transparent)]
        pub struct $name(pub crate::Label);

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Self {
                Self(crate::Label::new(value))
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self(crate::Label::default())
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl<'a> From<&'a str> for $name {
            fn from(given_str: &'a str) -> Self {
                Self::new(given_str)
            }
        }

        impl core::str::FromStr for $name {
            type Err = core::convert::Infallible;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(s.into())
            }
        }
    };
}

pub(crate) use resource_name;
