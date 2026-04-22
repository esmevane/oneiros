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

        impl<T> From<T> for $name
        where
            T: AsRef<str>,
        {
            fn from(given_str: T) -> Self {
                Self::new(given_str.as_ref())
            }
        }

        impl core::str::FromStr for $name {
            type Err = core::convert::Infallible;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self::new(s))
            }
        }
    };
}

pub(crate) use resource_name;
