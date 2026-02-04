use leptos::*;
use leptos_router::use_navigate;

use crate::components::ui::{Button, Input, LanguageToggle};
use crate::providers::locale::{translate, use_locale};
use crate::providers::auth::{use_auth, User};

#[component]
pub fn Login() -> impl IntoView {
    let auth = use_auth();
    let locale = use_locale();
    let navigate = use_navigate();

    let demo_mode = option_env!("RUSTOK_DEMO_MODE").is_some();
    let (tenant, set_tenant) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(Option::<String>::None);

    let navigate_effect = navigate.clone();
    create_effect(move |_| {
        if auth.token.get().is_some() {
            navigate_effect("/dashboard", Default::default());
        }
    });

    let on_submit = move |_| {
        if tenant.get().is_empty() || email.get().is_empty() || password.get().is_empty() {
            set_error.set(Some(
                translate(locale.locale.get(), "login.errorRequired").to_string(),
            ));
            return;
        }

        if !demo_mode {
            set_error.set(Some(
                translate(locale.locale.get(), "login.errorDemoDisabled").to_string(),
            ));
            return;
        }

        auth.set_token
            .set(Some(format!("demo-token:{}", tenant.get())));
        auth.set_user.set(Some(User {
            id: "demo".to_string(),
            email: email.get(),
            name: Some("Администратор".to_string()),
            role: "admin".to_string(),
        }));
        navigate("/dashboard", Default::default());
    };

    view! {
        <section class="auth-grid">
            <aside class="auth-visual">
                <span class="badge">{move || translate(locale.locale.get(), "login.badge")}</span>
                <h1>{move || translate(locale.locale.get(), "login.heroTitle")}</h1>
                <p>
                    {move || translate(locale.locale.get(), "login.heroSubtitle")}
                </p>
                <div>
                    <p>
                        <strong>{move || translate(locale.locale.get(), "login.heroListTitle")}</strong>
                    </p>
                    <p>{move || translate(locale.locale.get(), "login.heroListSubtitle")}</p>
                </div>
            </aside>
            <div class="auth-form">
                <div class="auth-card">
                    <div>
                        <h2>{move || translate(locale.locale.get(), "login.title")}</h2>
                        <p>{move || translate(locale.locale.get(), "login.subtitle")}</p>
                    </div>
                    <LanguageToggle />
                    <Show when=move || error.get().is_some()>
                        <div class="alert">{move || error.get().unwrap_or_default()}</div>
                    </Show>
                    <Input
                        value=tenant
                        set_value=set_tenant
                        placeholder="demo"
                        label=move || translate(locale.locale.get(), "login.tenantLabel").to_string()
                    />
                    <Input
                        value=email
                        set_value=set_email
                        placeholder="admin@rustok.io"
                        label=move || translate(locale.locale.get(), "login.emailLabel").to_string()
                    />
                    <Input
                        value=password
                        set_value=set_password
                        placeholder="••••••••"
                        type_="password"
                        label=move || translate(locale.locale.get(), "login.passwordLabel").to_string()
                    />
                    <Button on_click=on_submit class="w-full">
                        {move || translate(locale.locale.get(), "login.submit")}
                    </Button>
                    <Show when=move || demo_mode>
                        <a class="secondary-link" href="/dashboard">
                            {move || translate(locale.locale.get(), "login.demoLink")}
                        </a>
                    </Show>
                </div>
                <p style="margin:0; color:#64748b;">
                    {move || translate(locale.locale.get(), "login.footer")}
                </p>
            </div>
        </section>
    }
}
