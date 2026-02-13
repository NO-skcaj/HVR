use yew::prelude::*;
use yew_router::prelude::*;
pub mod components;

use crate::{ 
    components::home::*,
    components::blogs::*, 
    components::program::*, 
    components::reading::*
};


    // ***************************************
// MAIN ENTRY POINT
// ***************************************

fn main() {
    yew::Renderer::<App>::new().render();
}

// ***************************************
// ROUTING
// ***************************************

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/program")]
    Program,
    #[at("/blog")]
    Blog,
    #[at("/blog/:id")]
    BlogPost { id: String },
    #[at("/reading")]
    Reading,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// ***************************************
// PAGES - NOT FOUND
// ***************************************

#[function_component(NotFound)]
fn not_found() -> Html {
    html! {
        <div class="not-found-page">
            <div class="container">
                <div class="not-found-content">
                    <h1>{"404"}</h1>
                    <h2>{"Page Not Found"}</h2>
                    <p>{"The page you're looking for doesn't exist."}</p>
                    <a href="/" class="btn btn-primary">{"Go Home"}</a>
                </div>
            </div>
        </div>
    }
}

// ***************************************
// MAIN APP COMPONENT
// ***************************************

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Program => html! { <Program /> },
        Route::Blog => html! { <Blog /> },
        Route::BlogPost { id } => html! { <BlogPost {id} /> },
        Route::Reading => html! { <Reading /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

pub struct App();

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        app()
    }
}

fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="app">
                <Navbar />
                <main class="main-content">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    } 
}