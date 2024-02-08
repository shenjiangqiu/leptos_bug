use axum::Router;
use leptos::get_configuration;
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos_bug::{app::App, fallback::file_and_error_handler, state::AppState};

#[tokio::main]
pub async fn main() {

    // 3. Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options,
        routes: routes.clone(),
    };

    let app = Router::new()
        .leptos_routes(&app_state, routes.clone(), App)
        .fallback(file_and_error_handler)
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
