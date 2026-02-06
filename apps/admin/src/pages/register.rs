use leptos::prelude::*;

use crate::components::ui::{Button, Input, LanguageToggle};
use crate::providers::locale::{translate, use_locale};

#[component]
pub fn Register() -> impl IntoView {
    let locale = use_locale();

    let (tenant, set_tenant) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (name, set_name) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (invite_token, set_invite_token) = signal(String::new());
    let (verification_email, set_verification_email) = signal(String::new());
    let (error, set_error) = signal(Option::<String>::None);
    let (status, set_status) = signal(Option::<String>::None);

    let on_submit = move |_| {
        if tenant.get().is_empty() || email.get().is_empty() || password.get().is_empty() {
            set_error.set(Some(
                translate(locale.locale.get(), "register.errorRequired").to_string(),
            ));
            set_status.set(None);
            return;
        }

        set_error.set(None);
        set_status.set(Some(
            translate(locale.locale.get(), "register.success").to_string(),
        ));
    };

    let on_accept_invite = move |_| {
        if invite_token.get().is_empty() {
            set_error.set(Some(
                translate(locale.locale.get(), "register.inviteRequired").to_string(),
            ));
            set_status.set(None);
            return;
        }

        set_error.set(None);
        set_status.set(Some(
            translate(locale.locale.get(), "register.inviteAccepted").to_string(),
        ));
    };

    let on_resend_verification = move |_| {
        if verification_email.get().is_empty() {
            set_error.set(Some(
                translate(locale.locale.get(), "register.verifyRequired").to_string(),
            ));
            set_status.set(None);
            return;
        }

        set_error.set(None);
        set_status.set(Some(
            translate(locale.locale.get(), "register.verifySent").to_string(),
        ));
    };

    view! {
        <section class="grid min-h-screen grid-cols-1 lg:grid-cols-[1.2fr_1fr]">
            <aside class="flex flex-col justify-center gap-6 bg-[radial-gradient(circle_at_top_left,#1e3a8a,#0f172a)] p-12 text-white lg:p-16">
                <span class="inline-flex w-fit items-center rounded-full bg-white/10 px-3 py-1 text-xs font-semibold text-white/80">
                    {move || translate(locale.locale.get(), "register.badge")}
                </span>
                <h1 class="text-4xl font-semibold">{move || translate(locale.locale.get(), "register.heroTitle")}</h1>
                <p class="text-lg text-white/80">{move || translate(locale.locale.get(), "register.heroSubtitle")}</p>
                <div class="grid gap-2">
                    <p class="text-sm font-semibold">
                        {move || translate(locale.locale.get(), "register.heroListTitle")}
                    </p>
                    <p class="text-sm text-white/75">
                        {move || translate(locale.locale.get(), "register.heroListSubtitle")}
                    </p>
                </div>
            </aside>
            <div class="flex flex-col justify-center gap-7 bg-slate-50 p-12 lg:p-20">
                <div class="flex flex-col gap-5 rounded-3xl bg-white p-8 shadow-[0_24px_60px_rgba(15,23,42,0.12)]">
                    <div>
                        <h2 class="text-2xl font-semibold">
                            {move || translate(locale.locale.get(), "register.title")}
                        </h2>
                        <p class="text-slate-500">
                            {move || translate(locale.locale.get(), "register.subtitle")}
                        </p>
                    </div>
                    <div class="flex items-center justify-between gap-3 text-sm text-slate-600">
                        <span>{move || translate(locale.locale.get(), "register.languageLabel")}</span>
                        <LanguageToggle />
                    </div>
                    <Show when=move || error.get().is_some()>
                        <div class="rounded-xl bg-red-100 px-4 py-2 text-sm text-red-700">
                            {move || error.get().unwrap_or_default()}
                        </div>
                    </Show>
                    <Show when=move || status.get().is_some()>
                        <div class="rounded-xl bg-emerald-100 px-4 py-2 text-sm text-emerald-700">
                            {move || status.get().unwrap_or_default()}
                        </div>
                    </Show>
                    <Input
                        value=tenant
                        set_value=set_tenant
                        placeholder="demo"
                        label=move || translate(locale.locale.get(), "register.tenantLabel")
                    />
                    <Input
                        value=email
                        set_value=set_email
                        placeholder="admin@rustok.io"
                        label=move || translate(locale.locale.get(), "register.emailLabel")
                    />
                    <Input
                        value=name
                        set_value=set_name
                        placeholder="Alex Morgan"
                        label=move || translate(locale.locale.get(), "register.nameLabel")
                    />
                    <Input
                        value=password
                        set_value=set_password
                        placeholder="••••••••"
                        type_="password"
                        label=move || translate(locale.locale.get(), "register.passwordLabel")
                    />
                    <p class="text-sm text-slate-500">
                        {move || translate(locale.locale.get(), "register.passwordHint")}
                    </p>
                    <Button on_click=on_submit class="w-full">
                        {move || translate(locale.locale.get(), "register.submit")}
                    </Button>
                    <div class="flex justify-between gap-3 text-sm">
                        <a class="text-blue-600 hover:underline" href="/login">
                            {move || translate(locale.locale.get(), "register.loginLink")}
                        </a>
                        <a class="text-blue-600 hover:underline" href="/reset">
                            {move || translate(locale.locale.get(), "register.resetLink")}
                        </a>
                    </div>
                </div>

                <div class="flex flex-col gap-5 rounded-3xl bg-white p-8 shadow-[0_24px_60px_rgba(15,23,42,0.12)]">
                    <div>
                        <h3 class="text-lg font-semibold">
                            {move || translate(locale.locale.get(), "register.inviteTitle")}
                        </h3>
                        <p class="text-slate-500">
                            {move || translate(locale.locale.get(), "register.inviteSubtitle")}
                        </p>
                    </div>
                    <Input
                        value=invite_token
                        set_value=set_invite_token
                        placeholder="INVITE-2024-0001"
                        label=move || translate(locale.locale.get(), "register.inviteLabel")
                    />
                    <Button
                        on_click=on_accept_invite
                        class="w-full border border-indigo-200 bg-transparent text-blue-600 hover:bg-blue-50"
                    >
                        {move || translate(locale.locale.get(), "register.inviteSubmit")}
                    </Button>
                </div>

                <div class="flex flex-col gap-5 rounded-3xl bg-white p-8 shadow-[0_24px_60px_rgba(15,23,42,0.12)]">
                    <div>
                        <h3 class="text-lg font-semibold">
                            {move || translate(locale.locale.get(), "register.verifyTitle")}
                        </h3>
                        <p class="text-slate-500">
                            {move || translate(locale.locale.get(), "register.verifySubtitle")}
                        </p>
                    </div>
                    <Input
                        value=verification_email
                        set_value=set_verification_email
                        placeholder="admin@rustok.io"
                        label=move || translate(locale.locale.get(), "register.verifyLabel")
                    />
                    <Button
                        on_click=on_resend_verification
                        class="w-full border border-indigo-200 bg-transparent text-blue-600 hover:bg-blue-50"
                    >
                        {move || translate(locale.locale.get(), "register.verifySubmit")}
                    </Button>
                </div>
            </div>
        </section>
    }
}
