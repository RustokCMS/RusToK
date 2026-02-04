use leptos::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locale {
    En,
    Ru,
}

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

pub fn provide_locale_context() {
    let initial_locale = load_locale_from_storage().unwrap_or(Locale::Ru);
    let (locale, set_locale) = create_signal(initial_locale);

    create_effect(move |_| {
        if let Some(storage) = local_storage() {
            let _ = storage.set_item("rustok-admin-locale", locale.get().code());
        }
    });

    provide_context(LocaleContext { locale, set_locale });
}

pub fn use_locale() -> LocaleContext {
    use_context::<LocaleContext>().expect("LocaleContext not found")
}

pub fn translate(locale: Locale, key: &str) -> &'static str {
    match locale {
        Locale::En => translate_en(key),
        Locale::Ru => translate_ru(key),
    }
}

fn translate_en(key: &str) -> &'static str {
    match key {
        "app.dashboard" => "Dashboard",
        "app.users" => "Users",
        "login.badge" => "Admin Foundation",
        "login.heroTitle" => "RusToK Control Center",
        "login.heroSubtitle" => "Manage tenants, modules, and content in one place. Configurable access, fast actions, and transparent analytics.",
        "login.heroListTitle" => "Included in v1.0",
        "login.heroListSubtitle" => "Login, roles, activity charts, and module control.",
        "login.title" => "Sign in to admin",
        "login.subtitle" => "Enter your credentials to access the control panel.",
        "login.tenantLabel" => "Tenant Slug",
        "login.emailLabel" => "Email",
        "login.passwordLabel" => "Password",
        "login.submit" => "Continue",
        "login.demoLink" => "Open demo dashboard →",
        "login.footer" => "Need access? Contact a security administrator to activate.",
        "login.errorRequired" => "Please fill in all fields",
        "login.errorDemoDisabled" => "Demo login is disabled. Use server auth or enable RUSTOK_DEMO_MODE.",
        "dashboard.subtitle" => "RusToK system summary: key metrics and quick module access.",
        "dashboard.logout" => "Log out",
        "dashboard.createTenant" => "Create tenant",
        "dashboard.stats.tenants" => "Active tenants",
        "dashboard.stats.tenantsHint" => "+3 in a week",
        "dashboard.stats.modules" => "Modules in work",
        "dashboard.stats.modulesHint" => "Commerce, Blog, Tickets",
        "dashboard.stats.latency" => "API response time",
        "dashboard.stats.latencyHint" => "−14% in 7 days",
        "dashboard.stats.queue" => "Queue tasks",
        "dashboard.stats.queueHint" => "2 critical",
        "dashboard.activity.title" => "Latest activity",
        "dashboard.activity.tenant" => "New tenant",
        "dashboard.activity.tenantDetail" => "Nordic Supply",
        "dashboard.activity.tenantTime" => "2 minutes ago",
        "dashboard.activity.module" => "Module",
        "dashboard.activity.moduleDetail" => "Commerce updated to v1.0.3",
        "dashboard.activity.moduleTime" => "20 minutes ago",
        "dashboard.activity.security" => "Security",
        "dashboard.activity.securityDetail" => "Editor roles updated",
        "dashboard.activity.securityTime" => "1 hour ago",
        "dashboard.activity.content" => "Content",
        "dashboard.activity.contentDetail" => "Promo page published",
        "dashboard.activity.contentTime" => "Today",
        "dashboard.quick.title" => "Quick actions",
        "dashboard.quick.security" => "Run security audit",
        "dashboard.quick.users" => "User management",
        "dashboard.quick.metrics" => "Check API metrics",
        "dashboard.quick.roles" => "Generate role report",
        "users.title" => "Users",
        "users.subtitle" => "REST and GraphQL API preview. Provide admin token and tenant slug.",
        "users.refresh" => "Refresh",
        "users.access.title" => "Access settings",
        "users.access.hint" => "REST /api/auth/me requires a Bearer token. GraphQL users needs users:list permission.",
        "users.access.token" => "Bearer token",
        "users.access.tenant" => "Tenant slug",
        "users.rest.title" => "REST: /api/auth/me",
        "users.rest.loading" => "Loading...",
        "users.rest.pending" => "Waiting for response...",
        "users.rest.unauthorized" => "Unauthorized: check the token.",
        "users.rest.error" => "REST error:",
        "users.graphql.title" => "GraphQL: users",
        "users.graphql.total" => "Total users:",
        "users.graphql.email" => "Email",
        "users.graphql.name" => "Name",
        "users.graphql.role" => "Role",
        "users.graphql.status" => "Status",
        "users.graphql.error" => "GraphQL error:",
        "users.graphql.network" => "Network error.",
        "users.graphql.unauthorized" => "Unauthorized: check the token.",
        "users.noName" => "No name",
        "users.placeholderDash" => "—",
        _ => key,
    }
}

fn translate_ru(key: &str) -> &'static str {
    match key {
        "app.dashboard" => "Дашборд",
        "app.users" => "Пользователи",
        "login.badge" => "Admin Foundation",
        "login.heroTitle" => "RusToK Control Center",
        "login.heroSubtitle" => "Управляйте тенантами, модулями и контентом в одном месте. Настраиваемый доступ, быстрые действия и прозрачная аналитика.",
        "login.heroListTitle" => "Входит в v1.0",
        "login.heroListSubtitle" => "Логин, роли, графики активности и контроль модулей.",
        "login.title" => "Вход в админ-панель",
        "login.subtitle" => "Введите рабочие данные для доступа к панели управления.",
        "login.tenantLabel" => "Tenant Slug",
        "login.emailLabel" => "Email",
        "login.passwordLabel" => "Пароль",
        "login.submit" => "Продолжить",
        "login.demoLink" => "Перейти в демонстрационный дашборд →",
        "login.footer" => "Нужен доступ? Напишите администратору безопасности для активации.",
        "login.errorRequired" => "Заполните все поля",
        "login.errorDemoDisabled" => "Демо-вход отключен. Используйте серверную аутентификацию или включите RUSTOK_DEMO_MODE.",
        "dashboard.subtitle" => "Сводка системы RusToK: ключевые метрики и быстрый доступ к модулям.",
        "dashboard.logout" => "Выйти",
        "dashboard.createTenant" => "Создать тенант",
        "dashboard.stats.tenants" => "Активные тенанты",
        "dashboard.stats.tenantsHint" => "+3 за неделю",
        "dashboard.stats.modules" => "Модули в работе",
        "dashboard.stats.modulesHint" => "Commerce, Blog, Tickets",
        "dashboard.stats.latency" => "Время отклика API",
        "dashboard.stats.latencyHint" => "−14% за 7 дней",
        "dashboard.stats.queue" => "Задач в очереди",
        "dashboard.stats.queueHint" => "2 критичных",
        "dashboard.activity.title" => "Последняя активность",
        "dashboard.activity.tenant" => "Новый тенант",
        "dashboard.activity.tenantDetail" => "Nordic Supply",
        "dashboard.activity.tenantTime" => "2 минуты назад",
        "dashboard.activity.module" => "Модуль",
        "dashboard.activity.moduleDetail" => "Commerce обновлён до v1.0.3",
        "dashboard.activity.moduleTime" => "20 минут назад",
        "dashboard.activity.security" => "Безопасность",
        "dashboard.activity.securityDetail" => "Обновлены роли редакторов",
        "dashboard.activity.securityTime" => "1 час назад",
        "dashboard.activity.content" => "Контент",
        "dashboard.activity.contentDetail" => "Запущена публикация промо-страницы",
        "dashboard.activity.contentTime" => "Сегодня",
        "dashboard.quick.title" => "Быстрые действия",
        "dashboard.quick.security" => "Запустить аудит безопасности",
        "dashboard.quick.users" => "Управление пользователями",
        "dashboard.quick.metrics" => "Проверить метрики API",
        "dashboard.quick.roles" => "Сформировать отчёт по ролям",
        "users.title" => "Пользователи",
        "users.subtitle" => "Демонстрация работы с REST и GraphQL API. Введите токен администратора и tenant slug для доступа.",
        "users.refresh" => "Обновить",
        "users.access.title" => "Параметры доступа",
        "users.access.hint" => "REST эндпоинт /api/auth/me требует Bearer-токен. GraphQL users требует permissions users:list.",
        "users.access.token" => "Bearer token",
        "users.access.tenant" => "Tenant slug",
        "users.rest.title" => "REST: /api/auth/me",
        "users.rest.loading" => "Загрузка...",
        "users.rest.pending" => "Ожидание ответа...",
        "users.rest.unauthorized" => "Нет доступа: проверьте токен.",
        "users.rest.error" => "Ошибка REST:",
        "users.graphql.title" => "GraphQL: users",
        "users.graphql.total" => "Всего пользователей:",
        "users.graphql.email" => "Email",
        "users.graphql.name" => "Имя",
        "users.graphql.role" => "Роль",
        "users.graphql.status" => "Статус",
        "users.graphql.error" => "Ошибка GraphQL:",
        "users.graphql.network" => "Сетевая ошибка.",
        "users.graphql.unauthorized" => "Нет доступа: проверьте токен.",
        "users.noName" => "Без имени",
        "users.placeholderDash" => "—",
        _ => key,
    }
}

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window().and_then(|window| window.local_storage().ok().flatten())
}

fn load_locale_from_storage() -> Option<Locale> {
    let storage = local_storage()?;
    let value = storage.get_item("rustok-admin-locale").ok().flatten()?;
    Some(Locale::from_code(&value))
}
