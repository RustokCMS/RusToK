use leptos::prelude::*;

use crate::components::ui::{Button, Input};
use crate::providers::locale::{translate, use_locale};

struct SessionItem {
    device: &'static str,
    ip: &'static str,
    last_active_key: &'static str,
    status_key: &'static str,
}

struct LoginEvent {
    timestamp_key: &'static str,
    ip: &'static str,
    status_key: &'static str,
}

#[component]
pub fn Security() -> impl IntoView {
    let locale = use_locale();

    let (current_password, set_current_password) = signal(String::new());
    let (new_password, set_new_password) = signal(String::new());
    let (status, set_status) = signal(Option::<String>::None);

    let on_change_password = move |_| {
        if current_password.get().is_empty() || new_password.get().is_empty() {
            set_status.set(Some(
                translate(locale.locale.get(), "security.passwordRequired").to_string(),
            ));
            return;
        }

        set_status.set(Some(
            translate(locale.locale.get(), "security.passwordUpdated").to_string(),
        ));
    };

    let sessions = vec![
        SessionItem {
            device: "MacBook Pro · Chrome",
            ip: "91.204.12.8",
            last_active_key: "security.session.lastActiveNow",
            status_key: "security.session.active",
        },
        SessionItem {
            device: "iPhone 15 · Safari",
            ip: "31.54.102.3",
            last_active_key: "security.session.lastActiveYesterday",
            status_key: "security.session.idle",
        },
        SessionItem {
            device: "Windows · Edge",
            ip: "213.87.44.19",
            last_active_key: "security.session.lastActiveWeek",
            status_key: "security.session.inactive",
        },
    ];

    let history = vec![
        LoginEvent {
            timestamp_key: "security.history.timestamp.latest",
            ip: "91.204.12.8",
            status_key: "security.history.success",
        },
        LoginEvent {
            timestamp_key: "security.history.timestamp.prev",
            ip: "31.54.102.3",
            status_key: "security.history.success",
        },
        LoginEvent {
            timestamp_key: "security.history.timestamp.fail",
            ip: "81.23.119.52",
            status_key: "security.history.failed",
        },
    ];

    view! {
        <section class="px-10 py-8">
            <header class="mb-6 flex flex-wrap items-start justify-between gap-4">
                <div>
                    <span class="inline-flex items-center rounded-full bg-slate-200 px-3 py-1 text-xs font-semibold text-slate-600">
                        {move || translate(locale.locale.get(), "security.badge")}
                    </span>
                    <h1 class="mt-2 text-2xl font-semibold">
                        {move || translate(locale.locale.get(), "security.title")}
                    </h1>
                    <p class="mt-2 text-sm text-slate-500">
                        {move || translate(locale.locale.get(), "security.subtitle")}
                    </p>
                </div>
                <div class="flex flex-wrap items-center gap-3">
                    <Button
                        on_click=move |_| {}
                        class="border border-indigo-200 bg-transparent text-blue-600 hover:bg-blue-50"
                    >
                        {move || translate(locale.locale.get(), "security.signOutAll")}
                    </Button>
                </div>
            </header>

            <div class="grid gap-6 lg:grid-cols-2">
                <div class="grid gap-4 rounded-2xl bg-white p-6 shadow-[0_18px_36px_rgba(15,23,42,0.08)]">
                    <h3 class="text-lg font-semibold">
                        {move || translate(locale.locale.get(), "security.passwordTitle")}
                    </h3>
                    <p class="text-sm text-slate-500">
                        {move || translate(locale.locale.get(), "security.passwordSubtitle")}
                    </p>
                    <Input
                        value=current_password
                        set_value=set_current_password
                        placeholder="••••••••"
                        type_="password"
                        label=move || translate(locale.locale.get(), "security.currentPasswordLabel")
                    />
                    <Input
                        value=new_password
                        set_value=set_new_password
                        placeholder="••••••••"
                        type_="password"
                        label=move || translate(locale.locale.get(), "security.newPasswordLabel")
                    />
                    <p class="text-sm text-slate-500">
                        {move || translate(locale.locale.get(), "security.passwordHint")}
                    </p>
                    <Button on_click=on_change_password class="w-full">
                        {move || translate(locale.locale.get(), "security.passwordSubmit")}
                    </Button>
                    <Show when=move || status.get().is_some()>
                        <div class="rounded-xl bg-emerald-100 px-4 py-2 text-sm text-emerald-700">
                            {move || status.get().unwrap_or_default()}
                        </div>
                    </Show>
                </div>

                <div class="grid gap-4 rounded-2xl bg-white p-6 shadow-[0_18px_36px_rgba(15,23,42,0.08)]">
                    <h3 class="text-lg font-semibold">
                        {move || translate(locale.locale.get(), "security.sessionsTitle")}
                    </h3>
                    <p class="text-sm text-slate-500">
                        {move || translate(locale.locale.get(), "security.sessionsSubtitle")}
                    </p>
                    <div class="grid gap-3">
                        {sessions
                            .into_iter()
                            .map(|session| {
                                view! {
                                    <div class="flex items-center justify-between gap-4 border-b border-slate-200 py-3 last:border-b-0">
                                        <div>
                                            <strong>{session.device}</strong>
                                            <p class="text-sm text-slate-500">
                                                {move || translate(locale.locale.get(), "security.sessionIp")} ": "
                                                {session.ip}
                                            </p>
                                        </div>
                                        <div class="grid gap-1 text-right">
                                            <span class="inline-flex items-center justify-center rounded-full bg-slate-200 px-2.5 py-1 text-xs text-slate-600">
                                                {move || translate(locale.locale.get(), session.status_key)}
                                            </span>
                                            <span class="text-xs text-slate-400">
                                                {move || translate(locale.locale.get(), session.last_active_key)}
                                            </span>
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>

                <div class="grid gap-4 rounded-2xl bg-white p-6 shadow-[0_18px_36px_rgba(15,23,42,0.08)]">
                    <h3 class="text-lg font-semibold">
                        {move || translate(locale.locale.get(), "security.historyTitle")}
                    </h3>
                    <p class="text-sm text-slate-500">
                        {move || translate(locale.locale.get(), "security.historySubtitle")}
                    </p>
                    <div class="grid gap-3">
                        {history
                            .into_iter()
                            .map(|event| {
                                view! {
                                    <div class="flex items-center justify-between gap-4 border-b border-slate-200 py-3 last:border-b-0">
                                        <div>
                                            <strong>
                                                {move || translate(locale.locale.get(), event.timestamp_key)}
                                            </strong>
                                            <p class="text-sm text-slate-500">
                                                {move || translate(locale.locale.get(), "security.sessionIp")} ": "
                                                {event.ip}
                                            </p>
                                        </div>
                                        <span class="inline-flex items-center rounded-full bg-slate-200 px-2.5 py-1 text-xs text-slate-600">
                                            {move || translate(locale.locale.get(), event.status_key)}
                                        </span>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
            </div>
        </section>
    }
}
