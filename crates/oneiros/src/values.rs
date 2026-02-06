mod label {
    #[derive(Clone, Debug, serde::Serialize)]
    #[serde(transparent)]
    pub(crate) struct Label(pub String);

    impl Label {
        pub(crate) fn new(label: impl AsRef<str>) -> Self {
            Self(label.as_ref().into())
        }
    }

    impl core::fmt::Display for Label {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }
}

mod id {
    #[derive(Clone, Copy, Debug, serde::Serialize)]
    #[serde(transparent)]
    pub(crate) struct Id(pub uuid::Uuid);

    impl Id {
        pub(crate) fn new() -> Self {
            Self(uuid::Uuid::now_v7())
        }
    }
}

pub(crate) use id::Id;
pub(crate) use label::Label;
