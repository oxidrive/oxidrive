use std::{pin::Pin, sync::LazyLock};

use bytes::Bytes;
use futures::{stream::StreamExt, Stream};
use infer::Infer;

pub const DEFAULT_BINARY_TYPE: &str = "application/octet-stream";

static INFER: LazyLock<Infer> = LazyLock::new(|| {
    let mut infer = Infer::new();
    infer.add("text/svg", "svg", is_svg);
    infer
});

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
    match INFER.get(content) {
        Some(typ) => typ.mime_type(),
        None => DEFAULT_BINARY_TYPE,
    }
}

fn is_svg(buf: &[u8]) -> bool {
    infer::text::is_xml(buf) && {
        let text = String::from_utf8_lossy(buf);
        let second_line = text.trim().lines().nth(1).unwrap();
        eprintln!("checking whether {text} is an svg: {second_line}");
        second_line.starts_with("<svg")
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::jpeg("image/jpeg", &[0xFF, 0xD8, 0xFF, 0xAA])]
    #[case::svg(
        "text/svg",
        br#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg
   version="1.1"
"#
    )]
    fn it_infers_the_content_type(#[case] expected: &str, #[case] content: &[u8]) {
        check!(detect(content) == expected);
    }
}
