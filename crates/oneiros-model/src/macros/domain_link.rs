macro_rules! domain_link {
    ($linkable:ident => $wrapper:ident) => {
        impl oneiros_link::AsLink for $linkable {
            type Linkable = $wrapper;

            fn as_link(&self) -> Result<$wrapper, LinkError> {
                Ok($wrapper::$linkable(self.clone()))
            }
        }

        #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
        pub enum $wrapper {
            $linkable($linkable),
        }

        impl oneiros_link::Linkable for $wrapper {}
    };
}

pub(crate) use domain_link;
