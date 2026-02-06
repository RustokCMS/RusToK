use leptos::prelude::*;
use leptos_router::components::{Outlet, Route, Router, Routes};
use leptos_router::path;
use leptos_i18n::I18nContextProvider;

use crate::components::protected_route::ProtectedRoute;
use crate::pages::{
    dashboard::Dashboard, login::Login, not_found::NotFound, profile::Profile, register::Register,
    reset::ResetPassword, security::Security, user_details::UserDetails, users::Users,
};
use crate::providers::auth::provide_auth_context;
use crate::providers::locale::provide_locale_context;

#[component]
pub fn App() -> impl IntoView {
    provide_auth_context();
    let locale = provide_locale_context();

    view! {
        <I18nContextProvider locale=locale.locale>
            <Router>
                <main class="min-h-screen bg-slate-100 text-slate-900 font-sans">
                    // <script src="https://cdn.tailwindcss.com"></script> - REMOVED: using Trunk build
                    <Routes fallback=|| view! { <NotFound /> }>
                        <Route path=path!("/login") view=Login />
                        <Route path=path!("/register") view=Register />
                        <Route path=path!("/reset") view=ResetPassword />

                        <Route path=path!("") view=ProtectedRoute>
                            <Route path=path!("/dashboard") view=Dashboard />
                            <Route path=path!("/profile") view=Profile />
                            <Route path=path!("/security") view=Security />
                            <Route path=path!("/users") view=Users />
                            <Route path=path!("/users/:id") view=UserDetails />
                            <Route path=path!("") view=Dashboard />
                        </Route>

                        <Route path=path!("/*") view=NotFound />
                    </Routes>
                </main>
            </Router>
        </I18nContextProvider>
    }
}
