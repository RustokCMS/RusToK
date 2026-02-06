use leptos::prelude::*;
use leptos::web_sys;

pub use crate::i18n::Locale;
use crate::i18n;

impl Locale {
    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "en" => Locale::En,
            _ => Locale::Ru,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::Ru => "ru",
        }
    }
}

#[derive(Clone)]
pub struct LocaleContext {
    pub locale: ReadSignal<Locale>,
    pub set_locale: WriteSignal<Locale>,
}

pub fn provide_locale_context() -> LocaleContext {
    let initial_locale = load_locale_from_storage().unwrap_or(Locale::Ru);
    let (locale, set_locale) = signal(initial_locale);

    Effect::new(move |_| {
        if let Some(storage) = local_storage() {
            let _ = storage.set_item("rustok-admin-locale", locale.get().code());
        }
    });

    let context = LocaleContext { locale, set_locale };
    provide_context(context.clone());
    context
}

pub fn use_locale() -> LocaleContext {
    use_context::<LocaleContext>().expect("LocaleContext not found")
}

pub fn translate(key: &str) -> String {
    i18n::translate(key)
}

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window().and_then(|window| window.local_storage().ok().flatten())
}

fn load_locale_from_storage() -> Option<Locale> {
    let storage = local_storage()?;
    let value = storage.get_item("rustok-admin-locale").ok().flatten()?;
    Some(Locale::from_code(&value))
}
