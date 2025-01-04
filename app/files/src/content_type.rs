use std::pin::Pin;

use bytes::Bytes;
use futures::{stream::StreamExt, Stream};

pub const DEFAULT_BINARY_TYPE: &str = "application/octet-stream";

pub async fn detect_from_stream<S, E>(stream: S) -> (impl Stream<Item = Result<Bytes, E>>, String)
where
    S: Stream<Item = Result<Bytes, E>> + Unpin,
    E: 'static,
{
    let mut stream = stream.peekable();

    let s = Pin::new(&mut stream);
    let ct = async move {
        let buf = s.peek().await?.as_ref().ok()?;
        Some(detect(buf))
    }
    .await
    .unwrap_or(DEFAULT_BINARY_TYPE)
    .to_string();

    (stream, ct)
}

pub fn detect(content: &[u8]) -> &str {
    match infer::get(content) {
        Some(typ) => typ.mime_type(),
        None => DEFAULT_BINARY_TYPE,
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::jpeg("image/jpeg", &[0xFF, 0xD8, 0xFF, 0xAA])]
    fn it_infers_the_content_type(#[case] expected: &str, #[case] content: &[u8]) {
        check!(detect(content) == expected);
    }
}
