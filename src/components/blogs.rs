use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use serde::{Deserialize, Serialize};

// ***************************************
// DATA STRUCTURES - BLOG
// ***************************************

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct BlogPostData {
    pub id: String,
    pub title: String,
    pub date: String,
    pub preview: String,
    pub content: Vec<String>,
}

pub fn get_blog_posts() -> Vec<BlogPostData> {
    vec![
        BlogPostData {
            id: "origins-neoliberalism".to_string(),
            title: "The Origins of Neoliberalism".to_string(),
            date: "February 4, 2026".to_string(),
            preview: "".to_string(),
            content: vec![
                "In it's most abstract essence, Neoliberalism is the necessary dialectical evolution of 40s 50s social and economic conservatism, ie the bigots (racists), national chauvinists, and industrialists.".to_string(),
                "As opportunist ideology progresses it goes through periods of hiding and periods of full honesty. Neoliberalism is that period of hiding that came after Fascist movements in the pre-war era.".to_string(),
                "It throws out all of the previous eras social programs (that of FDR) and replaces it with Laissez Faire capitalism of the Austrian school of economic thought.".to_string(),
                "The basic idea around the ideology was that markets and actors therein should be able to make unregulated and planless decisions based on the machinations of the market.".to_string(),
                "This was a complete betrayal of the very successful Keneysian economic theory of FDR and the progressive movement.".to_string(),
                "The new Neoliberalism also came with a patent brand of liberal \"personal responsibility\" where systemic problems were never blamed for the environmental misfortunes of a given individual, and instead, the individual was blamed for their problems, even if they had no hand in the matter.".to_string(),
                "Functionally, the Neoliberals, under the revived Austrian school of thought (though now called the Chicago school of thought), fought for the rights of the owning class, like in Chile with the installation of Pinochet; and conversely, progressives, and leftists of that era, fought for the collective rights of workers under trade unions.".to_string(),
                "The synthesis of those ideas became the modern pro-capitalist Democratic party, and the continued Neoliberals (along with neocons aka war hawks) into the Republican party.".to_string(),
                "The Democrats dropped their progressive wing and became stark Neoliberals with a pretty face while Republicans synthesized their party with Libertarian and Neocon ideas, ultimately culminating in modern conservative thought.".to_string(),
                "Neoliberalism is, ultimately, the legacy of Ronald Reagan, Margaret Thatcher, and Milton Friedman, causing untold harm to the workers and their quality of lives, killing millions in the search for the Infinite Accumulation of wealth, and the spreading of economic \"freedoms\" and \"democracy.\"".to_string(),
                "You can think about it the same as the era of opportunist hiding before the Neoliberals, the Dixiecrats who touted cries of \"States\' Rights\" and the familiar liberal response of \"States' Rights to do what?\" With the obvious implication of slavery or some modern realization of socially chauvinistic economic exploitation or social exclusion. In the modern case, \"Spreading what freedom?\" With the implication of the freedom to have one's natural resources ruthlessly extracted.".to_string(),
                "The origins of Neoliberalism as the dominant ideology chiefly come from free market liberals (neoclassical economists etc) after WW2 and the establishment (and subsequent end) of the Bretton Woods system. Then, the response to the depowering of the US dollar, we pegged it to oil, tying US financial interests solely with the interests of American oil. Through a number of systems, we enforced the \"petrodollar\" where oil acted similarly to gold pre-Nixon.".to_string(),
                "So, the US started fighting wars over oil for its global Hegemony as part of a movement called Neoconservatism which called for Pinochet-esque interventions in places who are against US interests.".to_string(),
            ],
        },
        BlogPostData {
            id: "basic-class".to_string(),
            title: "Abstracting Class".to_string(),
            date: "January 30, 2026".to_string(),
            preview: "".to_string(),
            content: vec![
                "The majority of hierarchies are extremely similar to one another. The hierarchies of economics, society, and culture are all separate but they all contribute to what we call class.".to_string(),
                "Of course there's the obvious economic element: the rich, haves class who control and exploit the working class who is exploited. in that same way you have the social element which contains many different sub-elements like sexuality, opinions, and various other things. ".to_string(),
                "The last element being the culture element which is a bit easier to define than the other two being the culture you grew up in, the religion you follow and the general set of outward morals that you follow. ".to_string(),
                "I think that class is a term that can define your place in various different hierarchies. For example: if you are in the worker's class than you are below the owner's class in the economic hierarchy. ".to_string(),
                "In the same way, if you're in the heterosexual class you are above homosexuals in that hierarchy, and these hierarchies are perpetuated by a state, because the state is an organ of class rule. It's the same thing with culture, if the dominant culture is white Anglo-Saxon Protestantism, then the state is going to work for the benefit of that class. ".to_string(),
                "There's exceptions to this rule of class you can be a part of the political ruling class, but that doesn't mean that you are part of the ruling classes of other hierarchies (e.g. a gay black man in congress or a transgender billionaire). ".to_string(),
                "Class is a broad term that can define groups of people in various hierarchies. All of which culminates in a \"general hierarchy\" that the state enforces.".to_string(),
            ],
        },
        BlogPostData {
            id: "what-is-the-state".to_string(),
            title: "What is the State?".to_string(),
            date: "January 30, 2026".to_string(),
            preview: "".to_string(),
            content: vec![
                "The entire idea of a state is based on the repression of one class by another (ill write a much more lengthy essay on my interpretation of class later). The current US state is ruled by a political ruling class, those who we vote in to be our oppressors and by the economic elite who control our gov't.".to_string(),
                "In this sense, it is those members of that class dictating the actions of the state. It is the dictatorship of the economic ruling class, ie the bourgeoisie. This state is hardly representative of the people, it is the directive of larger economic interests; all of which have a say at the table, which we will never see.".to_string(),
                "The plans and schemes of the ruling class lead to greater and greater power grabs by that class and as such we, the people, are becoming more and more oppressed. In short, the state is a force of class repression for the betterment of a class that we are not part of.".to_string(),
                "So, the state does not seek to meet our needs. Only to the extent at which the state deems it necessary does the state need to work as to secure its legitimacy. What we want is a new state, a state that works at the behest of the people.".to_string(),
                "We want a state that is ruled by the working class where the working class dictate what happens through democratic means, where a small elite of people cannot decide for the nation as a whole, especially not for the sake of greed. What we want is a true democracy, a dictatorship of the working class, ie the proletariat.".to_string(),
                "To really make a drastic and lasting change in the workings of the state-machine there is only one way to do so. As a system works to perpetuate itself, it is only logical that to change it into something else it will require the use of force and violence.".to_string(),
                "However, this violence must be directed and made minimal. As the Chinese did the Puyi, the decapitation of pre-existing state power does not have to be literal.".to_string(),
                "The system cannot be reformed, and the ready made state-machine cannot be appropriated by a new class of workers who would them become the new political ruling class, continuing the same system that they revolted against. The ready made state machine must be smashed in it's entirety.".to_string(),
                "A state for and of the people must be created from the start for the interests of the proletariat. Then, we can start the work of creating a new state that seeks to destroy unjust hierarchies and create economic and political democracy. This is the dictatorship of the proletariat in the sense that we want it.".to_string(),
                "Not as a single man dictatorship, as in the liberal sense, but in the sense of a new state of the proletariat, where they dictate that happens through true democratic means.".to_string(),
            ],
        },
    ]
}

// ***************************************
// PAGES - BLOG POST
// ***************************************

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub id: String,
}

#[function_component(BlogPost)]
pub fn blog_post(props: &BlogPostProps) -> Html {
    let posts = get_blog_posts();
    let post = posts.iter().find(|p| p.id == props.id);

    match post {
        Some(post) => render_post(post),
        None => html! {
            <div class="blog-post-page">
                <div class="container">
                    <div class="not-found">
                        <h1>{"Post Not Found"}</h1>
                        <p>{"The blog post you're looking for doesn't exist."}</p>
                        <a href="/blog" class="btn btn-primary">{"Back to Blog"}</a>
                    </div>
                </div>
            </div>
        },
    }
}

fn render_post(post: &BlogPostData) -> Html {
    html! {
        <div class="blog-post-page">
            <article class="container blog-post-container">
                <header class="blog-post-header">
                    <div class="blog-post-meta">
                        <span class="blog-post-date">{&post.date}</span>
                    </div>
                    <h1 class="blog-post-title">{&post.title}</h1>
                </header>

                <div class="blog-post-content">
                    {post.content.iter().map(|paragraph| html! {
                        <p>{paragraph}</p>
                    }).collect::<Html>()}
                </div>

                <footer class="blog-post-footer">
                    <a href="/blog" class="back-link">{"← Back to All Posts"}</a>
                </footer>
            </article>
        </div>
    }
}

// ***************************************
// PAGES - BLOG LISTING
// ***************************************

#[function_component(Blog)]
pub fn blog() -> Html {
    let posts = get_blog_posts();

    html! {
        <div class="blog-page">
            <div class="container">
                <header class="page-header">
                    <h1>{"Blog"}</h1>
                    <p class="page-subtitle">
                        {"Updates, insights, and perspectives on local governance and community issues"}
                    </p>
                </header>

                <div class="blog-grid">
                    {posts.iter().map(|post| html! {
                        <article class="blog-card">
                            <div class="blog-card-content">
                                <div class="blog-meta">
                                    <span class="blog-date">{&post.date}</span>
                                </div>
                                <h2 class="blog-title">
                                    <Link<Route> to={Route::BlogPost { id: post.id.clone() }}>
                                        {&post.title}
                                    </Link<Route>>
                                </h2>
                                <p class="blog-preview">{&post.preview}</p>
                                <Link<Route> 
                                    to={Route::BlogPost { id: post.id.clone() }} 
                                    classes="read-more"
                                >
                                    {"Read More →"}
                                </Link<Route>>
                            </div>
                        </article>
                    }).collect::<Html>()}
                </div>
            </div>
        </div>
    }
}