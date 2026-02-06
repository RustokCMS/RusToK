use leptos_i18n::{t, use_i18n};

leptos_i18n::load_locales!();

pub use self::Locale;

pub fn translate(key: &str) -> String {
    let i18n = use_i18n();
    t!(i18n, key)
}
