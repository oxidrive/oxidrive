use core::fmt;
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

use axum::http::{HeaderName, HeaderValue, Method};
use serde::{
    Deserialize, Deserializer,
    de::{SeqAccess, Visitor},
};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer, ExposeHeaders};

#[derive(Clone, Debug, Deserialize)]
pub struct CorsConfig {
    #[serde(default)]
    allow_credentials: bool,

    #[serde(default, deserialize_with = "string_or_seq")]
    allow_headers: AnyOr<Vec<HttpHeaderName>>,

    #[serde(default, deserialize_with = "string_or_seq")]
    allow_methods: AnyOr<Vec<HttpMethod>>,

    #[serde(default, deserialize_with = "string_or_seq")]
    allow_origins: AnyOr<Vec<HttpHeaderValue>>,

    #[serde(default, deserialize_with = "string_or_seq")]
    expose_headers: AnyOr<Vec<HttpHeaderName>>,

    #[serde(default)]
    allow_private_networks: bool,
}

impl From<&CorsConfig> for CorsLayer {
    fn from(cfg: &CorsConfig) -> Self {
        let allowed_headers = match cfg.allow_headers {
            AnyOr::Any => AllowHeaders::any(),
            AnyOr::Value(ref allowed_headers) => AllowHeaders::list(
                allowed_headers
                    .iter()
                    .map(|HttpHeaderName(header)| header.clone()),
            ),
        };

        let allowed_methods = match cfg.allow_methods {
            AnyOr::Any => AllowMethods::any(),
            AnyOr::Value(ref allowed_methods) => AllowMethods::list(
                allowed_methods
                    .iter()
                    .map(|HttpMethod(method)| method.clone()),
            ),
        };

        let allowed_origins = match cfg.allow_origins {
            AnyOr::Any => AllowOrigin::any(),
            AnyOr::Value(ref allowed_origins) => AllowOrigin::list(
                allowed_origins
                    .iter()
                    .map(|HttpHeaderValue(origin)| origin.clone()),
            ),
        };

        let exposed_headers = match cfg.expose_headers {
            AnyOr::Any => ExposeHeaders::any(),
            AnyOr::Value(ref exposed_headers) => ExposeHeaders::list(
                exposed_headers
                    .iter()
                    .map(|HttpHeaderName(headers)| headers.clone()),
            ),
        };

        CorsLayer::new()
            .allow_credentials(cfg.allow_credentials)
            .allow_headers(allowed_headers)
            .allow_methods(allowed_methods)
            .allow_origin(allowed_origins)
            .expose_headers(exposed_headers)
            .allow_private_network(cfg.allow_private_networks)
    }
}

#[derive(Clone, Debug)]
enum AnyOr<T> {
    Any,
    Value(T),
}

impl<T> FromStr for AnyOr<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase() != "any" {
            return Err(format!(
                "invalid CORS configuration '{s}': should be either a {} or the literal string 'any'",
                std::any::type_name::<T>()
            ));
        }

        Ok(Self::Any)
    }
}

impl<'de, T> Deserialize<'de> for AnyOr<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Self::Value)
    }
}

impl<T: Default> Default for AnyOr<T> {
    fn default() -> Self {
        Self::Value(T::default())
    }
}

#[derive(Clone, Debug)]
struct HttpHeaderName(HeaderName);

impl<'de> Deserialize<'de> for HttpHeaderName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let header = String::deserialize(deserializer)?;
        let header = HeaderName::from_lowercase(header.to_lowercase().as_bytes())
            .map_err(<D::Error as serde::de::Error>::custom)?;
        Ok(Self(header))
    }
}

#[derive(Clone, Debug)]
struct HttpMethod(Method);

impl<'de> Deserialize<'de> for HttpMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let method = String::deserialize(deserializer)?;
        let method = Method::from_bytes(method.as_bytes())
            .map_err(<D::Error as serde::de::Error>::custom)?;
        Ok(Self(method))
    }
}

#[derive(Clone, Debug)]
struct HttpHeaderValue(HeaderValue);

impl<'de> Deserialize<'de> for HttpHeaderValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let header = String::deserialize(deserializer)?;
        let header =
            HeaderValue::from_str(&header).map_err(<D::Error as serde::de::Error>::custom)?;
        Ok(Self(header))
    }
}

fn string_or_seq<'de, T, TE, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = TE>,
    TE: Display + Debug,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T, TE> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = TE>,
        TE: Display + Debug,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: serde::de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_seq<M>(self, map: M) -> Result<T, M::Error>
        where
            M: SeqAccess<'de>,
        {
            Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
