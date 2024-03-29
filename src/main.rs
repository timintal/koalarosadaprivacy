mod privacy_page;

use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/*path")]
    Privacy { path: String },
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <><h1>{ "Privacy Policies:" }</h1>
            <Link<Route> to={Route::Privacy{path:"Drop_Craft".to_string()}}>{"Drop Craft"}</Link<Route>>
            </>
        },
        Route::Privacy { path } => privacy_page::get_privacy_page(path.replace("_", " ").as_str()),

        Route::NotFound => html! { <h1>{ "404. Page Not Found" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    // privacy_page::get_privacy_page("Drop Craft")
    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}