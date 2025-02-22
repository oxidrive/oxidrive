use std::{pin::Pin, sync::LazyLock};

use bytes::Bytes;
use futures::{Stream, stream::StreamExt};
use infer::Infer;

pub const DEFAULT_BINARY_TYPE: &str = "application/octet-stream";

static INFER: LazyLock<Infer> = LazyLock::new(|| {
    let mut infer = Infer::new();
    infer.add("text/svg", "svg", is_svg);
    infer
});

pub async fn detect_from_stream<S, E>(
    name: &str,
    stream: S,
) -> (impl Stream<Item = Result<Bytes, E>> + use<S, E>, String)
where
    S: Stream<Item = Result<Bytes, E>> + Unpin,
    E: 'static,
{
    let mut stream = stream.peekable();

    let s = Pin::new(&mut stream);
    let ct = async move {
        let buf = s.peek().await?.as_ref().ok()?;
        detect(name, buf)
    }
    .await
    .unwrap_or(DEFAULT_BINARY_TYPE.into());

    (stream, ct)
}

pub fn detect(name: &str, content: &[u8]) -> Option<String> {
    if let Some(typ) = INFER.get(content) {
        return Some(typ.mime_type().into());
    }

    if String::from_utf8(content.to_vec()).is_ok() {
        let typ = mime_guess::from_path(name).first_or_text_plain();
        return Some(typ.essence_str().into());
    }

    None
}

fn is_svg(buf: &[u8]) -> bool {
    infer::text::is_xml(buf) && {
        let text = String::from_utf8_lossy(buf);
        let second_line = text.trim().lines().nth(1).unwrap();
        second_line.starts_with("<svg")
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::jpeg("test.jpg", "image/jpeg", &[0xFF, 0xD8, 0xFF, 0xAA])]
    #[case::svg(
        "test.svg",
        "text/svg",
        br#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg
   version="1.1"
"#
    )]
    #[case::rust(
        "test.rs",
        "text/x-rust",
        br#"
use hello_world;

fn main() {}
"#
    )]
    #[case::text("test", "text/plain", b"Hello world!")]
    fn it_infers_the_content_type(
        #[case] name: &str,
        #[case] expected: &str,
        #[case] content: &[u8],
    ) {
        check!(detect(name, content).unwrap() == expected);
    }
}
