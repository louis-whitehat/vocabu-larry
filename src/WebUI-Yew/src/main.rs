mod api;
mod models;
mod views;

use yew::prelude::*;
use yew_router::prelude::*;

use views::{exam::ExamView, login::LoginView, logs::LogsView, score::ScoreView};

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Login,
    #[at("/exam/:user/:dictionary")]
    Exam { user: String, dictionary: String },
    #[at("/score/:user")]
    Score { user: String },
    #[at("/logs")]
    Logs,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Login => html! { <LoginView /> },
        Route::Exam { user, dictionary } => {
            html! { <ExamView user={user} dictionary={dictionary} /> }
        }
        Route::Score { user } => html! { <ScoreView user={user} /> },
        Route::Logs => html! { <LogsView /> },
        Route::NotFound => html! { <Redirect<Route> to={Route::Login} /> },
    }
}

#[function_component(AppShell)]
fn app_shell() -> Html {
    let current_route = use_route::<Route>();
    let show_home = !matches!(current_route.as_ref(), Some(Route::Login));
    let show_logs = !matches!(current_route.as_ref(), Some(Route::Logs));

    html! {
        <div class="app-shell">
            <header class="app-toolbar">
                <Link<Route> to={Route::Login} classes="brand-mark">{"Vocabu-Larry"}</Link<Route>>

                <nav class="toolbar-actions" aria-label="Global navigation">
                    if show_home {
                        <Link<Route> to={Route::Login} classes="toolbar-action">
                            <svg viewBox="0 0 24 24" aria-hidden="true">
                                <path d="M3 10.5 12 3l9 7.5M6.75 8.75V21h10.5V8.75M10 21v-6h4v6" />
                            </svg>
                        </Link<Route>>
                    }

                    if show_logs {
                        <Link<Route> to={Route::Logs} classes="toolbar-action">
                            <svg viewBox="0 0 24 24" aria-hidden="true">
                                <path d="M6 3h9l5 5v13a1 1 0 0 1-1 1H6a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2Zm8 1.5V9h4.5M8 13h8M8 17h8M8 9h3" />
                            </svg>
                        </Link<Route>>
                    }
                </nav>
            </header>

            <main class="page-frame">
                <Switch<Route> render={switch} />
            </main>
        </div>
    }
}

#[function_component(Root)]
fn root() -> Html {
    html! {
        <BrowserRouter>
            <AppShell />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
