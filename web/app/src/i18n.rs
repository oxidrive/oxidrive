use std::{borrow::Cow, rc::Rc};

use dioxus::hooks::{use_context, use_context_provider};
use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
pub use unic_langid::langid;
use unic_langid::LanguageIdentifier;

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
            langid!($lang),
            concat!($kind, sep!(), $name),
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

static DEFAULT_LOCALE: LanguageIdentifier = langid!("en");
static LOCALES: &[LanguageIdentifier] = &[langid!("en")];

type Translations<'a> = &'a [(LanguageIdentifier, &'a str, &'a str)];

static TRANSLATIONS: Translations = &[
    translation!("en", "page", "home.ftl"),
    translation!("en", "page", "setup.ftl"),
];

pub fn init() -> Localizer {
    let requested_locales = web_sys::window()
        .expect("window not found")
        .navigator()
        .languages()
        .into_iter()
        .filter_map(|lang| lang.as_string())
        .filter_map(|lang| lang.parse().ok())
        .collect::<Vec<LanguageIdentifier>>();

    let localizer = init_with(&requested_locales, TRANSLATIONS, true);
    use_context_provider(|| localizer)
}

fn init_with(
    requested_locales: &[LanguageIdentifier],
    translations: Translations,
    isolating: bool,
) -> Localizer {
    let selected_locales = negotiate_languages(
        requested_locales,
        LOCALES,
        Some(&DEFAULT_LOCALE),
        NegotiationStrategy::Filtering,
    )
    .into_iter()
    .cloned()
    .collect::<Vec<_>>();

    let bundle = init_bundle(selected_locales, translations, isolating);
    Localizer { bundle }
}

fn init_bundle(
    langs: Vec<LanguageIdentifier>,
    translations: Translations,
    isolating: bool,
) -> Rc<FluentBundle<FluentResource>> {
    let mut bundle = FluentBundle::new(langs.clone());
    bundle.set_use_isolating(isolating);

    for (lang, id, content) in translations
        .iter()
        .filter(|(lang, _, _)| langs.contains(lang))
    {
        let resource = FluentResource::try_new((*content).into())
            .unwrap_or_else(|_| panic!("failed to parse {lang}/{id}"));
        bundle
            .add_resource(resource)
            .unwrap_or_else(|_| panic!("failed to add resource {lang}/{id} to bundle"));
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

#[macro_export]
macro_rules! loc_args {
    ( $($key:expr => $value:expr),* ) => {
        {
            let mut args: fluent_bundle::FluentArgs = fluent_bundle::FluentArgs::new();
            $(
                args.set($key, $value);
            )*
            args
        }
    };
}

#[cfg(test)]
mod tests {

    use assert2::check;
    use rstest::rstest;
    use unic_langid::{langid, LanguageIdentifier};

    use crate::i18n::{init_with, Translations};

    #[rstest]
    #[case::normal(&[langid!("en-US"), langid!("en")])]
    #[case::default_fallback(&[langid!("de-AT"), langid!("fr-FR"), langid!("fr-CA")])]
    fn it_localizes_an_attribute(#[case] locales: &[LanguageIdentifier]) {
        let translations: Translations = &[(
            langid!("en"),
            "test.ftl",
            r#"
test = Unused
  .msg = hello {$name}!
"#,
        )];

        let i18n = init_with(locales, translations, false);
        check!(i18n.bundle.locales == &[langid!("en")]);

        let args = loc_args!["name" => "world"];
        let localized = i18n.localize_with("test.msg", &args);
        check!(localized == "hello world!");
    }
}
