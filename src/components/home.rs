use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

// ***************************************
// COMPONENTS - NAVBAR
// ***************************************

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar">
            <div class="navbar-container">
                <Link<Route> to={Route::Home} classes="navbar-logo">
                    <span class="logo-text">{"Hardin Valley Reform"}</span>
                </Link<Route>>
                
                <div class="navbar-links">
                    <Link<Route> to={Route::Program} classes="nav-link">
                        {"Program"}
                    </Link<Route>>
                    <Link<Route> to={Route::Blog} classes="nav-link">
                        {"Blog"}
                    </Link<Route>>
                    <Link<Route> to={Route::Reading} classes="nav-link">
                        {"Reading"}
                    </Link<Route>>
                </div>
            </div>
        </nav>
    }
}

// ***************************************
// PAGES - HOME
// ***************************************

#[function_component(Home)]
pub fn home() -> Html {

    html! {
        <>
        <div class="home-page">
            <section class="hero">
                <div class="hero-content">
                    <h1 class="hero-title">{"Hardin Valley Reform"}</h1>
                    <p class="hero-subtitle">
                        {"Building a stronger community through principled action and civic engagement"}
                    </p>
                    <div class="hero-cta">
                        <Link<Route> to={Route::Program} classes="btn btn-primary">{"Learn About Our Program"}</Link<Route>>
                        <Link<Route> to={Route::Blog}    classes="btn btn-primary">{"Read Our Blog"}</Link<Route>>
                    </div>
                </div>
            </section>

            <section class="intro-section">
                <div class="container">
                    <h2>{"Our Mission"}</h2>
                    <p class="intro-text">
                        {"Hardin Valley Reform is a student organized movement dedicated to working within our community and in solidarity with others to advocate for Left-Progressive ideas and perspectives."}
                    </p>
                </div>
            </section>

            <section class="features">
                <div class="container">
                    <div class="feature-grid">
                        <div class="feature-card">
                            <h3>{"People First"}</h3>
                            <p>{"Prioritizing the needs and voices of the people, not those of the rich and powerful."}</p>
                        </div>
                        <div class="feature-card">
                            <h3>{"Labor Power"}</h3>
                            <p>{"While we aren't old enough to truely join the working force, we recognize the importance of labor unions and labor-oriented collective action in building a just society."}</p>
                        </div>
                        <div class="feature-card">
                            <h3>{"No More Planlessness"}</h3>
                            <p>{"For too long, the unplanned development of the national economy has left industries in directions that are against the interests of the people."}</p>
                        </div>
                    </div>
                </div>
            </section>
        </div>
        
        </>
    }
}