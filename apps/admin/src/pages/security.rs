use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};

use crate::api::{rest_get, rest_post, ApiError};
use crate::components::ui::{Button, Input};
use crate::providers::auth::use_auth;
use crate::providers::locale::translate;

#[derive(Clone, Deserialize)]
struct SessionItem {
    user_agent: Option<String>,
    ip_address: Option<String>,
    created_at: String,
    current: bool,
}

#[derive(Deserialize)]
struct SessionsResponse {
    sessions: Vec<SessionItem>,
}

#[derive(Clone, Deserialize)]
struct HistoryItem {
    user_agent: Option<String>,
    ip_address: Option<String>,
    created_at: String,
    status_key: Option<String>,
}

#[derive(Deserialize)]
struct HistoryResponse {
    sessions: Vec<HistoryItem>,
}

#[derive(Serialize)]
struct ChangePasswordParams {
    current_password: String,
    new_password: String,
}

#[derive(Deserialize)]
struct GenericStatus {}

#[component]
pub fn Security() -> impl IntoView {
    let auth = use_auth();

    let (current_password, set_current_password) = signal(String::new());
    let (new_password, set_new_password) = signal(String::new());
    let (status, set_status) = signal(Option::<String>::None);
    let (_error, set_error) = signal(Option::<String>::None);
    let (sessions, set_sessions) = signal(Vec::<SessionItem>::new());
    let (history, set_history) = signal(Vec::<HistoryItem>::new());

    let load_sessions = move || {
        let token = auth.token.get();
        let tenant_slug = auth.tenant_slug.get();
        let set_sessions = set_sessions;
        let set_error = set_error;

        spawn_local(async move {
            let result =
                rest_get::<SessionsResponse>("/api/auth/sessions", token, tenant_slug).await;
            match result {
                Ok(response) => {
                    set_error.set(None);
                    set_sessions.set(response.sessions);
                }
                Err(err) => {
                    let message = match err {
                        ApiError::Unauthorized => {
                            translate("errors.auth.unauthorized").to_string()
                        }
                        ApiError::Http(_) => {
                            translate("errors.http").to_string()
                        }
                        ApiError::Network => {
                            translate("errors.network").to_string()
                        }
                        ApiError::Graphql(_) => {
                            translate("errors.unknown").to_string()
                        }
                    };
                    set_error.set(Some(message));
                }
            }
        });
    };

    let load_history = move || {
        let token = auth.token.get();
        let tenant_slug = auth.tenant_slug.get();
        let set_history = set_history;
        let set_error = set_error;

        spawn_local(async move {
            let result = rest_get::<HistoryResponse>("/api/auth/history", token, tenant_slug).await;
            match result {
                Ok(response) => {
                    set_error.set(None);
                    set_history.set(response.sessions);
                }
                Err(err) => {
                    let message = match err {
                        ApiError::Unauthorized => {
                            translate("errors.auth.unauthorized").to_string()
                        }
                        ApiError::Http(_) => {
                            translate("errors.http").to_string()
                        }
                        ApiError::Network => {
                            translate("errors.network").to_string()
                        }
                        ApiError::Graphql(_) => {
                            translate("errors.unknown").to_string()
                        }
                    };
                    set_error.set(Some(message));
                }
            }
        });
    };

    let on_change_password = move |_| {
        if current_password.get().is_empty() || new_password.get().is_empty() {
            set_error.set(Some(
                translate("security.passwordRequired").to_string(),
            ));
            set_status.set(None);
            return;
        }

        let token = auth.token.get();
        let tenant_slug = auth.tenant_slug.get();
        if token.is_none() {
            set_error.set(Some(
                translate("errors.auth.unauthorized").to_string(),
            ));
            set_status.set(None);
            return;
        }

        let current_password_value = current_password.get();
        let new_password_value = new_password.get();
        let set_error = set_error;
        let set_status = set_status;

        spawn_local(async move {
            let result = rest_post::<ChangePasswordParams, GenericStatus>(
                "/api/auth/change-password",
                &ChangePasswordParams {
                    current_password: current_password_value,
                    new_password: new_password_value,
                },
                token,
                tenant_slug,
            )
            .await;

            match result {
                Ok(_) => {
                    set_error.set(None);
                    set_status.set(Some(
                        translate("security.signOutAll").to_string(),
                    ));
                }
                Err(err) => {
                    let message = match err {
                        ApiError::Unauthorized => {
                            translate("errors.auth.unauthorized").to_string()
                        }
                        ApiError::Http(_) => {
                            translate("errors.http").to_string()
                        }
                        ApiError::Network => {
                            translate("errors.network").to_string()
                        }
                        ApiError::Graphql(_) => {
                            translate("errors.unknown").to_string()
                        }
                    };
                    set_error.set(Some(message));
                    set_status.set(None);
                }
            }
        });
    };

    let on_sign_out_all = move |_: leptos::ev::MouseEvent| {
        let token = auth.token.get();
        let tenant_slug = auth.tenant_slug.get();
        let set_error = set_error;
        let set_status = set_status;

        spawn_local(async move {
            let result = rest_post::<serde_json::Value, GenericStatus>(
                "/api/auth/sessions/revoke-all",
                &serde_json::json!({}),
                token,
                tenant_slug,
            )
            .await;

            match result {
                Ok(_) => {
                    set_error.set(None);
                    set_status.set(Some(
                        translate("security.passwordUpdated").to_string(),
                    ));
                }
                Err(err) => {
                    let message = match err {
                        ApiError::Unauthorized => {
                            translate("errors.auth.unauthorized").to_string()
                        }
                        ApiError::Http(_) => {
                            translate("errors.http").to_string()
                        }
                        ApiError::Network => {
                            translate("errors.network").to_string()
                        }
                        ApiError::Graphql(_) => {
                            translate("errors.unknown").to_string()
                        }
                    };
                    set_error.set(Some(message));
                }
            }
        });
    };

    Effect::new(move |_| {
        load_sessions();
        load_history();
    });

    view! {
        <section class="px-10 py-8">
            <header class="mb-6 flex flex-wrap items-start justify-between gap-4">
                <div>
                    <span class="inline-flex items-center rounded-full bg-slate-200 px-3 py-1 text-xs font-semibold text-slate-600">
                        {move || translate("security.badge")}
                    </span>
                    <h1 class="mt-2 text-2xl font-semibold">
                        {move || translate("security.title")}
                    </h1>
                    <p class="mt-2 text-sm text-slate-500">
                        {move || translate("security.subtitle")}
                    </p>
                </div>
                <div class="flex flex-wrap items-center gap-3">
                    <Button
                        on_click=on_sign_out_all
                        class="border border-indigo-200 bg-transparent text-blue-600 hover:bg-blue-50"
                    >
                        {move || translate("security.signOutAll")}
                    </Button>
                </div>
            </header>

            <div class="grid gap-6 lg:grid-cols-2">
                <div class="grid gap-4 rounded-2xl bg-white p-6 shadow-[0_18px_36px_rgba(15,23,42,0.08)]">
                    <h3 class="text-lg font-semibold">
                        {move || translate("security.passwordTitle")}
                    </h3>
                    <p class="text-sm text-slate-500">
                        {move || translate("security.passwordSubtitle")}
                    </p>
                    <Input
                        value=current_password
                        set_value=set_current_password
                        placeholder="••••••••"
                        type_="password"
                        label=move || translate("security.currentPasswordLabel")
                    />
                    <Input
                        value=new_password
                        set_value=set_new_password
                        placeholder="••••••••"
                        type_="password"
                        label=move || translate("security.newPasswordLabel")
                    />
                    <p class="text-sm text-slate-500">
                        {move || translate("security.passwordHint")}
                    </p>
                    <Button on_click=on_change_password class="w-full">
                        {move || translate("security.passwordSubmit")}
                    </Button>
                    <Show when=move || status.get().is_some()>
                        <div class="rounded-xl bg-emerald-100 px-4 py-2 text-sm text-emerald-700">
                            {move || status.get().unwrap_or_default()}
                        </div>
                    </Show>
                </div>

                <div class="settings-card">
                    <h3>{move || translate("security.sessionsTitle")}</h3>
                    <p class="section-subtitle">{move || translate("security.sessionsSubtitle")}</p>
                    <div class="session-list">
                        {move || {
                            sessions
                                .get()
                                .into_iter()
                                .map(|session| {
                                    let label = session
                                        .user_agent
                                        .clone()
                                        .unwrap_or_else(|| "Unknown device".to_string());
                                    let ip = session
                                        .ip_address
                                        .clone()
                                        .unwrap_or_else(|| "Unknown IP".to_string());
                                    let status_label = if session.current {
                                        "Current"
                                    } else {
                                        "Other"
                                    };
                                    view! {
                                        <div class="session-item">
                                            <div>
                                                <strong>{label}</strong>
                                                <p class="form-hint">
                                                    {move || translate("security.sessionIp")}
                                                    {": "}
                                                    {ip}
                                                </p>
                                            </div>
                                            <div class="session-meta">
                                                <span class="status-pill">{status_label}</span>
                                                <span class="meta-text">{session.created_at}</span>
                                            </div>
                                        </div>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                </div>

                <div class="settings-card">
                    <h3>{move || translate("security.historyTitle")}</h3>
                    <p class="section-subtitle">{move || translate("security.historySubtitle")}</p>
                    <div class="session-list">
                        {move || {
                            history
                                .get()
                                .into_iter()
                                .map(|event| {
                                    let label = event
                                        .user_agent
                                        .clone()
                                        .unwrap_or_else(|| "Unknown device".to_string());
                                    let ip = event
                                        .ip_address
                                        .clone()
                                        .unwrap_or_else(|| "Unknown IP".to_string());
                                    let status_key = event
                                        .status_key
                                        .clone()
                                        .unwrap_or_else(|| "security.history.success".to_string());
                                    view! {
                                        <div class="session-item">
                                            <div>
                                                <strong>{label}</strong>
                                                <p class="form-hint">
                                                    {move || translate("security.sessionIp")}
                                                    {": "}
                                                    {ip}
                                                </p>
                                            </div>
                                            <span class="status-pill">{event.created_at}</span>
                                            <span class="inline-flex items-center rounded-full bg-slate-200 px-2.5 py-1 text-xs text-slate-600">
                                                {move || translate(status_key.as_str())}
                                            </span>
                                        </div>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                </div>
            </div>
        </section>
    }
}
