use yew::prelude::*;
use yew_router::prelude::*;
use std::cell::RefCell;
use postgrest::Postgrest;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha256;
pub mod components;

use crate::{
    components::{ 
        blogs::{Blog, BlogPost},
        home::{Home, Navbar}, 
        program::Program, 
        reading::Reading,
    }
};

// ***************************************
// GLOBAL VARIABLES
// ***************************************

thread_local! {
    pub static DATA_BASE: RefCell<(postgrest::Postgrest, String)> = RefCell::new((Postgrest::new("http://localhost:3000"), "".to_string()));
}

// ***************************************
// MAIN ENTRY POINT
// ***************************************

fn main() {
    dotenvy::dotenv().ok();


    DATA_BASE.with_borrow_mut(|db_k| {
        db_k.1 = generate_jwt(std::env::var("SECRET").unwrap_or("test_secret_that_is_at_least_32_characters_long6942".to_string()).as_str(), "web_anon");
        db_k.0 = Postgrest::new(std::env::var("DB_URL").unwrap_or("http://localhost:3000".to_string()));
    });

    yew::Renderer::<App>::new().render();
}

type HmacSha256 = Hmac<Sha256>;

fn generate_jwt(secret: &str, role: &str) -> String {
    // Header
    let header = r#"{"alg":"HS256","typ":"JWT"}"#;
    let header_b64 = URL_SAFE_NO_PAD.encode(header.as_bytes());
    
    // Payload - you might need to add 'exp', 'iat', 'iss' claims
    let payload = format!(r#"{{"role":"{}"}}"#, role);
    let payload_b64 = URL_SAFE_NO_PAD.encode(payload.as_bytes());
    
    // Create signing input
    let signing_input = format!("{}.{}", header_b64, payload_b64);
    
    // Create HMAC-SHA256 signature
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(signing_input.as_bytes());
    let signature_bytes = mac.finalize().into_bytes();
    
    // Base64 URL encode the signature
    let signature_b64 = URL_SAFE_NO_PAD.encode(&signature_bytes);
    
    // Construct final JWT
    format!("{}.{}", signing_input, signature_b64)
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
    BlogPost { id: i32 },
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
        self.app()
    }
}

impl App
{
    fn app(&self) -> Html
    {
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
}