pub fn translate_en(key: &str) -> Option<&'static str> {
    match key {
        "auth.badge" => Some("Admin Foundation"),
        "auth.heroTitle" => Some("RusToK Control Center"),
        "auth.heroSubtitle" => Some(
            "Manage tenants, modules, and content in one place. Configurable access, fast actions, and transparent analytics.",
        ),
        "auth.heroListTitle" => Some("Included in v1.0"),
        "auth.heroListSubtitle" => Some("Login, roles, activity charts, and module control."),
        "auth.title" => Some("Sign in to admin"),
        "auth.subtitle" => Some("Enter your credentials to access the control panel."),
        "auth.tenantLabel" => Some("Tenant Slug"),
        "auth.emailLabel" => Some("Email"),
        "auth.passwordLabel" => Some("Password"),
        "auth.submit" => Some("Continue"),
        "auth.demoLink" => Some("Open demo dashboard →"),
        "auth.footer" => Some("Need access? Contact a security administrator to activate."),
        "auth.errorRequired" => Some("Please fill in all fields"),
        "auth.errorDemoDisabled" => Some(
            "Demo login is disabled. Use server auth or enable RUSTOK_DEMO_MODE.",
        ),
        _ => None,
    }
}

pub fn translate_ru(key: &str) -> Option<&'static str> {
    match key {
        "auth.badge" => Some("Admin Foundation"),
        "auth.heroTitle" => Some("RusToK Control Center"),
        "auth.heroSubtitle" => Some(
            "Управляйте тенантами, модулями и контентом в одном месте. Настраиваемый доступ, быстрые действия и прозрачная аналитика.",
        ),
        "auth.heroListTitle" => Some("Входит в v1.0"),
        "auth.heroListSubtitle" => Some("Логин, роли, графики активности и контроль модулей."),
        "auth.title" => Some("Вход в админ-панель"),
        "auth.subtitle" => Some("Введите рабочие данные для доступа к панели управления."),
        "auth.tenantLabel" => Some("Tenant Slug"),
        "auth.emailLabel" => Some("Email"),
        "auth.passwordLabel" => Some("Пароль"),
        "auth.submit" => Some("Продолжить"),
        "auth.demoLink" => Some("Перейти в демонстрационный дашборд →"),
        "auth.footer" => Some(
            "Нужен доступ? Напишите администратору безопасности для активации.",
        ),
        "auth.errorRequired" => Some("Заполните все поля"),
        "auth.errorDemoDisabled" => Some(
            "Демо-вход отключен. Используйте серверную аутентификацию или включите RUSTOK_DEMO_MODE.",
        ),
        _ => None,
    }
}
