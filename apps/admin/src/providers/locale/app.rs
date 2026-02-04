pub fn translate_en(key: &str) -> Option<&'static str> {
    match key {
        "app.nav.dashboard" => Some("Dashboard"),
        "app.nav.users" => Some("Users"),
        _ => None,
    }
}

pub fn translate_ru(key: &str) -> Option<&'static str> {
    match key {
        "app.nav.dashboard" => Some("Дашборд"),
        "app.nav.users" => Some("Пользователи"),
        _ => None,
    }
}
