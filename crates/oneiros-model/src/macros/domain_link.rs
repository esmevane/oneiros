macro_rules! domain_link {
    ($linkable:ident => $wrapper:ident) => {
        impl oneiros_link::AsLink for $linkable {
            type Linkable = $wrapper;

            fn as_link(&self) -> Result<$wrapper, oneiros_link::LinkError> {
                Ok($wrapper::$linkable(self.clone()))
            }
        }

        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
        pub enum $wrapper {
            $linkable($linkable),
        }

        impl TryFrom<Link> for $wrapper {
            type Error = LinkError;

            fn try_from(link: Link) -> Result<Self, Self::Error> {
                Self::from_string(link.to_string())
            }
        }

        impl core::fmt::Display for $wrapper {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(
                    f,
                    "{}",
                    self.to_link_string()
                        .unwrap_or("Malformed link string".to_string())
                )
            }
        }

        impl oneiros_link::Linkable for $wrapper {}
    };
}

pub(crate) use domain_link;
