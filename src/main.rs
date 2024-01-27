mod privacy_page;

use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/privacy/*path")]
    Privacy { path: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home Page" }</h1> },
        Route::Privacy { path } => privacy_page::get_privacy_page(path.replace("_", " ").as_str()),

        Route::NotFound => html! { <h1>{ "404. Page Not Found" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    // privacy_page::get_privacy_page("Drop Craft")
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}