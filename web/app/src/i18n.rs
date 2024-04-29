use std::{borrow::Cow, rc::Rc};

use dioxus::hooks::{use_context, use_context_provider};
use fluent::{FluentArgs, FluentBundle, FluentResource};
use unic_langid::{langid, LanguageIdentifier};

#[cfg(not(windows))]
macro_rules! sep {
    () => {
        "/"
    };
}

#[cfg(windows)]
macro_rules! sep {
    () => {
        r#"\"#
    };
}

macro_rules! translation {
    ($lang: tt, $kind: tt, $name: tt) => {
        (
            concat!($lang, sep!(), $kind, sep!(), $name),
            include_str!(concat!(
                "translations",
                sep!(),
                $lang,
                sep!(),
                $kind,
                sep!(),
                $name
            )),
        )
    };
}

const EN_TRANSLATIONS: &[(&str, &str)] = &[
    translation!("en", "page", "home.ftl"),
    translation!("en", "page", "setup.ftl"),
];

pub fn init() -> Localizer {
    let bundle = init_lang(langid!("en"), EN_TRANSLATIONS);
    use_context_provider(|| Localizer { bundle })
}

fn init_lang(
    lang: LanguageIdentifier,
    sources: &[(&str, &str)],
) -> Rc<FluentBundle<FluentResource>> {
    let mut bundle = FluentBundle::new(vec![lang]);

    for (id, content) in sources {
        let resource = FluentResource::try_new((*content).into())
            .unwrap_or_else(|_| panic!("failed to parse {id}"));
        bundle
            .add_resource(resource)
            .unwrap_or_else(|_| panic!("failed to add resource {id} to bundle"));
    }

    Rc::new(bundle)
}

#[derive(Clone)]
pub struct Localizer {
    bundle: Rc<FluentBundle<FluentResource>>,
}

impl Localizer {
    pub fn localize(&self, key: &str) -> Cow<'_, str> {
        self.format(key, None)
    }

    #[allow(dead_code)] // it will be used, and it's used in tests but they don't trigger Clippy
    pub fn localize_with<'a>(&'a self, key: &str, args: &'a FluentArgs<'_>) -> Cow<'a, str> {
        self.format(key, Some(args))
    }

    fn format<'a>(&'a self, key: &str, args: Option<&'a FluentArgs<'_>>) -> Cow<'a, str> {
        let (key, attr) = if key.contains('.') {
            let mut parts = key.split('.');
            (parts.next().unwrap(), parts.next())
        } else {
            (key, None)
        };

        let msg = self
            .bundle
            .get_message(key)
            .unwrap_or_else(|| panic!("failed to retrieve translation {key}"));

        let pattern = attr
            .and_then(|attr| msg.get_attribute(attr))
            .map(|attr| attr.value())
            .or_else(|| msg.value())
            .unwrap_or_else(|| panic!("translation {key} has no value"));

        let mut errors = vec![];

        let translated = self.bundle.format_pattern(pattern, args, &mut errors);
        for error in errors {
            log::error!("failed to format translation {key}: {error}");
        }

        translated
    }
}

pub fn use_localizer() -> Localizer {
    use_context()
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use assert2::check;
    use fluent::{fluent_args, FluentBundle, FluentResource};
    use unic_langid::langid;

    use super::Localizer;

    #[test]
    fn it_localizes_an_attribute() {
        let resource = FluentResource::try_new(
            r#"
test = Unused
  .msg = hello {$name}!
"#
            .into(),
        )
        .unwrap();

        let mut bundle = FluentBundle::new(vec![langid!("en")]);
        // important for testing or strings will contain additional UTF-8 characters that fail the equality check
        bundle.set_use_isolating(false);

        bundle.add_resource(resource).unwrap();

        let i18n = Localizer {
            bundle: Rc::new(bundle),
        };
        let args = fluent_args!["name" => "world"];
        let localized = i18n.localize_with("test.msg", &args);
        check!(localized == "hello world!");
    }
}
