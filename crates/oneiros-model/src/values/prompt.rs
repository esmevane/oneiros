#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Prompt(pub String);

impl Prompt {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for Prompt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<T> for Prompt
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl core::str::FromStr for Prompt {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}
