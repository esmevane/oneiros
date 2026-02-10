#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Id(pub uuid::Uuid);

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl Id {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_nil()
    }
}
