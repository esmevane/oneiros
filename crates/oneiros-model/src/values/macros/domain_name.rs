macro_rules! domain_name {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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

        impl core::str::FromStr for $name {
            type Err = core::convert::Infallible;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self::new(s))
            }
        }
    };
}

pub(crate) use domain_name;
