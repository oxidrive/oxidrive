#[macro_export]
macro_rules! make_uuid_type {
    ($typ:ident, $macro_name:ident) => {
        #[derive(
            Default,
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            ::serde::Serialize,
            ::serde::Deserialize,
            Hash,
        )]
        #[serde(transparent)]
        pub struct $typ(::uuid::Uuid);

        impl From<::uuid::Uuid> for $typ {
            fn from(value: ::uuid::Uuid) -> Self {
                Self(value)
            }
        }

        pub(crate) mod macros {
            #[macro_export]
            macro_rules! $macro_name {
                ($uuid:literal) => {{
                    const OUTPUT: $typ = match $typ::try_parse($uuid) {
                        Ok(u) => u,
                        Err(_) => panic!("invalid uuid representation"),
                    };
                    OUTPUT
                }};
            }

            pub(crate) use $macro_name;
        }

        impl $typ {
            pub fn new() -> Self {
                Self(::uuid::Uuid::now_v7())
            }

            pub fn as_uuid(&self) -> ::uuid::Uuid {
                self.0
            }

            pub const fn try_parse(s: &'static str) -> Result<Self, ::uuid::Error> {
                match ::uuid::Uuid::try_parse(s) {
                    Ok(u) => Ok(Self(u)),
                    Err(e) => Err(e),
                }
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_nil()
            }
        }

        impl ::std::fmt::Display for $typ {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::str::FromStr for $typ {
            type Err = ::uuid::Error;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                Ok(Self(s.parse()?))
            }
        }

        impl ::std::convert::From<$typ> for String {
            fn from(value: $typ) -> Self {
                value.0.into()
            }
        }

        impl ::std::convert::From<$typ> for ::uuid::Uuid {
            fn from(value: $typ) -> Self {
                value.0
            }
        }
    };
}

#[macro_export]
macro_rules! make_error_wrapper {
    ($typ:ident) => {
        #[derive(Debug, thiserror::Error)]
        #[error("{message}")]
        pub struct $typ {
            message: String,
            source: Box<dyn std::error::Error + Send + Sync + 'static>,
            source_type: &'static str,
        }

        impl $typ {
            pub fn new(
                message: impl ToString,
                source: impl std::error::Error + Send + Sync + 'static,
            ) -> Self {
                Self {
                    message: message.to_string(),
                    source_type: std::any::type_name_of_val(&source),
                    source: Box::new(source) as Box<dyn std::error::Error + Send + Sync + 'static>,
                }
            }

            pub fn wrap(source: impl std::error::Error + Send + Sync + 'static) -> Self {
                Self::new(source.to_string(), source)
            }

            pub fn source_type(&self) -> &'static str {
                self.source_type
            }
        }
    };
}
