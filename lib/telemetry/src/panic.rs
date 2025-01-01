use std::panic::{Location, PanicHookInfo};

static TARGET: &str = "panic_hook";

pub fn install_panic_logger() {
    std::panic::set_hook(Box::new(|info| {
        let location = info
            .location()
            .expect("location should always be present in modern Rust versions");

        let stack = std::backtrace::Backtrace::force_capture();

        if tracing::enabled!(target: TARGET, tracing::Level::ERROR) {
            log_with_tracing(info, stack, location);
        } else {
            log_with_eprintln(info, stack, location);
        }
    }));
}

fn log_with_tracing(
    info: &PanicHookInfo<'_>,

    stack: std::backtrace::Backtrace,
    location: &Location<'_>,
) {
    tracing::error!(target: TARGET,
        panic = true,
        error.kind = "Panic",
        error.message = %info,
        error.stack = %stack,
        error.column = location.column(),
        error.line = location.line(),
        error.file = location.file(),
        "panic occurred"
    );
}

fn log_with_eprintln(
    info: &PanicHookInfo<'_>,
    stack: std::backtrace::Backtrace,
    location: &Location<'_>,
) {
    let line = JsonLine {
        error: &format!("{info}"),
        stack: &format!("{stack}"),
        line: location.line(),
        column: location.column(),
        file: location.file(),
    };
    eprintln!("{line}");
}

struct JsonLine<'a> {
    error: &'a str,
    stack: &'a str,
    line: u32,
    column: u32,
    file: &'a str,
}

impl std::fmt::Display for JsonLine<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        {
            write!(f, r#""level":"error","#)?;
            write!(f, r#""panic":true,"#)?;
            write!(f, r#""message":"panic occurred","#)?;
            write!(f, r#""error":{{"#)?;
            {
                write!(f, r#""kind":"Panic","#)?;
                write!(f, r#""message":"{}","#, self.error)?;
                write!(f, r#""stack":"{}","#, self.stack)?;
                write!(f, r#""line":{},"#, self.line)?;
                write!(f, r#""column":{},"#, self.column)?;
                write!(f, r#""file":"{}""#, self.file)?;
            }
            write!(f, r#"}}"#)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_some_panic_to_json() {
        let line = JsonLine {
            error: "test error",
            stack: "test stack",
            line: 1,
            column: 1,
            file: "src/example.rs:1:1",
        };

        let expected = serde_json::to_string(&serde_json::json!({
            "level": "error",
            "panic": true,
            "message": "panic occurred",
            "error": {
                "kind": "Panic",
                "message": line.error,
                "stack": line.stack,
                "line": line.line,
                "column": line.column,
                "file": line.file,
            },
        }))
        .unwrap();

        assert2::assert!(line.to_string() == expected);
    }
}
