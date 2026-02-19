use yew::prelude::*;
use yew_router::prelude::*;
use crate::{DATA_BASE, Route};
use serde::{Deserialize, Serialize};
use yew::platform::spawn_local;
use yew::use_effect_with;


// ***************************************
// DATA STRUCTURES - BLOG
// ***************************************

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct BlogPostData {
    pub id: i32,
    pub title: String,
    pub date: String,
    pub preview: String,
    pub content: String,
}

pub fn string_to_lines(paragraph: String) -> Vec<String> {
    paragraph.lines().map(|line| line.to_string()).collect()
}

pub fn lines_to_string(lines: Vec<String>) -> String {
    lines.join("\n")
}

pub fn get_blog_posts() -> Option<Vec<BlogPostData>> {
    Some(vec![
        BlogPostData {
            id: 0,
            title: "The Origins of Neoliberalism".to_string(),
            date: "February 4, 2026".to_string(),
            preview: "Neoliberalism is the dominant ideology in modern society, rooted in the post-WW2 economic and political shifts.".to_string(),
            content: lines_to_string(vec![
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
            ]),
        },
        BlogPostData {
            id: 1,
            title: "Abstracting Class".to_string(),
            date: "January 30, 2026".to_string(),
            preview: "Intersectionality and class in modern society".to_string(),
            content: lines_to_string(vec![
                "The majority of hierarchies are extremely similar to one another. The hierarchies of economics, society, and culture are all separate but they all contribute to what we call class.".to_string(),
                "Of course there's the obvious economic element: the rich, haves class who control and exploit the working class who is exploited. in that same way you have the social element which contains many different sub-elements like sexuality, opinions, and various other things. ".to_string(),
                "The last element being the culture element which is a bit easier to define than the other two being the culture you grew up in, the religion you follow and the general set of outward morals that you follow. ".to_string(),
                "I think that class is a term that can define your place in various different hierarchies. For example: if you are in the worker's class than you are below the owner's class in the economic hierarchy. ".to_string(),
                "In the same way, if you're in the heterosexual class you are above homosexuals in that hierarchy, and these hierarchies are perpetuated by a state, because the state is an organ of class rule. It's the same thing with culture, if the dominant culture is white Anglo-Saxon Protestantism, then the state is going to work for the benefit of that class. ".to_string(),
                "There's exceptions to this rule of class you can be a part of the political ruling class, but that doesn't mean that you are part of the ruling classes of other hierarchies (e.g. a gay black man in congress or a transgender billionaire). ".to_string(),
                "Class is a broad term that can define groups of people in various hierarchies. All of which culminates in a \"general hierarchy\" that the state enforces.".to_string(),
            ]),
        },
        BlogPostData {
            id: 2,
            title: "What is the State?".to_string(),
            date: "January 30, 2026".to_string(),
            preview: "Laws are threats made by the dominant socioeconomic-ethnic group in a given nation. It\'s just the promise of violence that\'s enacted and the police are basically an occupying army.".to_string(),
            content: lines_to_string(vec![
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
            ]),
        },
        BlogPostData {
            id: 3,
            title: "An Inevitability of Capital".to_string(),
            date: "February 18, 2026".to_string(),
            preview: "".to_string(),
            content: lines_to_string(vec![
                "Capitalism inevitably ends with an ultimate centralization of capital. The reasoning behind this is deceptively simple: in free markets, there are winners of the market and losers of the market.".to_string(),
                "The winners gain more market share, making more money, and further centralizes their given industry in a process called horizontal integration. Those losers no longer exist as market entities and are forced into poverty or to simply sell their labor like a worker.".to_string(),
                "As that process of centralization continues, governments work to stop it. Which is in all senses impractical and inevitably futile. Centralization of industries like this leads to exceedingly competitive large entities which follow cults of profit and of efficiency (either that or the entity becomes corrupt and inevitably fails due to that lack in efficiency).".to_string(),
                "That efficiency allows for further horizontal integration until they completely dominate a market and create monopoly. That does two things, it creates a monopoly of employment and a monopoly of the given good. Certain workers can only work for one company and a entire product or array of products comes from one source, causing a rise in the prices, a lowering in the quality, and a preditorial hunt for any prospective competitors.".to_string(),
                "This process is natural, and as I've explained, inevitable within a free market. Capital will centralize into large social enterprises with RnD departments and market researchers and all other sorts of workers who deal in the advancement of the product either for \"competiton\" within a multi-opoly or for the encouragement of more buying. These sects of the monopolistic capital institutions are not entirely evil.".to_string(),
                "Now, each and every one of these ventures is \"corrupted\" by the unholy search of unlimited and constant increase in profits. Corners must be cut, but the problem is that they don't always lose in the market. Monopoly is a fickle thing that destroys Adam Smith's ideas of the invisible hand (not that he could've really predicted less than 10 men controlling free markets).".to_string(), 
                "And thus capital accumulates, centralized, and decays into a never ending run of profit seeking self-destructive firms that rely on the state and the taxpayers money to fund the unlimited increase in their own profits. Now what is to be done here? The centralization of capital allows for an extremely efficient social appropriation of that capital, changing the benefactors from the rich owners to the people at large, eliminating a class and bolstering the population general.".to_string(),
                "Without the leeches, the institutions owned by the rich can serve for the sake of their industry and be extremely efficient in doing so due to the centralization of the industry. That socialization of industry is the chief focus of our political project. That is the goal upon which all others stand.".to_string(),
            ]),
        },
        BlogPostData {
            id: 4,
            title: "A Rant on Attacking Individuals and Not Systems".to_string(),
            date: "February 18, 2026".to_string(),
            preview: "Attacking individuals is that it doesn't solve the problem, it just replaces a real problem with one actor.".to_string(),
            content: lines_to_string(vec![
                "A trend I often see with people on the left from center left to far left is that they broadly blame and want to kill billionaires, or whatever pejorative they may use.".to_string(), 
                "I'll preface by saying that I really really truly believe that at least 90% of those people are complete scum of the Earth. However, there's an important issue with the framing that all billionaires are bad.".to_string(),
                "People exist within systems. People want to succeed within systems. Succeeding in our current system directly means the exploitation of others and the pushing down of others for ones own gain. CEOs, stock traders, politicians, and other members of the bourgeoisie see capitalism, and the market, as a game to win.".to_string(), 
                "But the truth is that you can never really win and they can never be satisfied so they must push further and further. And in order to do that they must commit crimes and do things that could make them the scum of the Earth.".to_string(),
                "General trends, especially around the advent of the Epstein files, show this very clearly. Not only the need to commit crimes to pursue political and economic, but also the ability to commit crimes without much reprecussion (as of writing).".to_string(), 
                "To say that *all* billionaires are bad is a slight systemic error as they are simply acting as expected within their given conditions. That being, an endlessly competitive and ruthless system which predicates itself on endless corruption and greed. Not having chains any social chains leads the most worst people ".to_string(), 
                "My thesis has been a bit all around so I'll clarify: capitalists exist and behave as expected as humans within a system, their crimes should be punished regardless, but we should not be surprised when the elites do something bad, illegal, exceedingly immoral etc.".to_string(),
                "I believe buy'n'large that if you remove capitalists from the system of winners and losers, the poor and the rich, they will become normal moral people (and if given enough time with no change to the system, more capitalists will replace those ones). However that's not really possible or practical in most cases.".to_string(), 
                "Kill them all but know that there's a world where you wouldn't have to.".to_string(),
                "".to_string(),
                "Addendum by a Contributor:".to_string(),
                "".to_string(),
                "Killing or prosecuting the rich would also inherently cause political instability. Even if their assets are already seized, there are always those who love to support those in power or those who ere previously in power.".to_string(), 
                "Not to mention the problem this will cause in a society - How can a socialist nation be the beacon of worker's rights and freedom if they simply just focus their oppression on another class?".to_string(), 
                "This is why the soviet union and china failed to spread their ideology, and the reason for why the red scare was so effective in the western world, and why the rich, despite it being arguably morally right to persecute them, should not be.".to_string(), 
                "Instead they should simply be re-integrated into the new system while being treated equally as anyone else. Does this mean they walk free? Yes, but it also spares the general population from oppression.".to_string(), 
            ]),
        },
        BlogPostData {
            id: 5,
            title: "The Problem with the Left".to_string(),
            date: "February 18, 2026".to_string(),
            preview: "".to_string(),
            content: lines_to_string(vec![
                "Chauvinism is the arbitrary belief that your person group is better than some other person group. Chauvinism realizes itself as racism, homophobia, classism, and nationalism/patriotism. All of which are intrinsic to reactionary and right wing theory. That being, the creation of an arbitrary class ingroup and the boosting of that ingroup via the state.".to_string(), 
                "In our ideas of the class struggle, we the people, the workers, are all equals. Because our social, racial, and economic conditions are decided arbitrarily per person, there is no rational reason to treat anyone differently because of the nature of their birth.".to_string(), 
                "A person is a person no matter their race, gender, sexuality, etc. So, why do we make such a big fuss and such a big distinction between the people of one country and that of another?".to_string(), 
                "Call me a dreamer (but I'm not the only one), but I solidly believe that all people have more in common with others and have a natural proclivity towards cooperation.".to_string(), "Don't be fooled by those who preach \"American Exceptionalism,\" or those who start taking about some white pride or black pride or asian pride.".to_string(), 
                "We, my friends, have the people's pride, of the legacy and history of hundreds of years of struggle against the oppressive forces.".to_string(), 
                "Considering all of this, your first assumption may be to completely disregard race as a pragmatic concept. You may then denounce the BPPFSD (Black Panther Party For Self Defense) and call them race realists.".to_string(), 
                "That would be another mistake. Over the years of Imperialist oppression, some oppressors targeted specific racial groups and thus their socioeconomic conditions are disproportionately much worse than other segments of the population.".to_string(), 
                "Of course, this isn't the fault of any individual black person or of any grouping of black people. Do not blame a person or group for the effects of a system that actively works to push them down.".to_string(), 
                "In this way, the struggles of the black population are different in it's severity than to, say, white people.".to_string(), 
                "So it makes sense that different organizations would form to fight different forms of oppression, as these forms of oppression arise not from just economic conditions but general social proclivities that are aided and ebeded by the \"general forms of oppression.\"".to_string(),
                "E.g. an organization of mostly whites will, necessarily, have a harder time dealing with racism than an organization of mostly black people.".to_string(), 
                "This is not an argument against solidarity. Solidarity is the logical next step: as we band with our brothers and sisters against oppression of the general stort, so shall we again band with them against other forms of oppression.".to_string(),
                
            ]),
        }
    ])
}

// ***************************************
// PAGES - BLOG POST
// ***************************************

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub id: i32,
}

#[component(BlogPost)]
pub fn blog_post(props: &BlogPostProps) -> Html {
    let counter = use_state(|| Vec::new());
    
    // Run on component mount
    {
        let counter = counter.clone();
        use_effect_with((), move |_| {
            let counter = counter.clone();
            spawn_local(async move {
                let mut __resp__: Option<postgrest::Builder> = None;
                DATA_BASE.with_borrow_mut(|__db_k__| {
                    __resp__ = Some(__db_k__.0
                        .from("blogpost")
                        .auth(__db_k__.1.clone())
                        .select("*"));
                });
                if let Some(builder) = __resp__ {
                    match builder.execute().await {
                        Ok(res) => {
                            if let Ok(text) = res.text().await {
                                counter.set(serde_json::from_str::<Vec<BlogPostData>>(&text).unwrap());
                            }
                        }
                        Err(_) => {counter.set(vec![BlogPostData{id: 0, title: "Error Loading Posts".to_string(), date: String::new(), preview: String::new(), content: String::new()}]);},
                    }
                }
            });
            || ()
        });
    }
    
    // let posts: Vec<BlogPostData> = counter.to_vec();
    let posts = get_blog_posts().unwrap();
    
    let post = posts.iter().find(|p| &p.id == &props.id);

    match post {
        Some(post) => render_post(post),
        None => html! {
            <div class="blog-post-page">
                <div class="container">
                    <div class="not-found">
                        <h1>{"Post Not Found"}</h1>
                        <p>{"The blog post you're looking for doesn't exist."}</p>
                        <Link<Route> to={Route::Blog} classes="btn btn-primary">{"Back to Blog"}</Link<Route>>
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
                    {string_to_lines(post.content.clone()).iter().map(|paragraph| html! {
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
    let counter = use_state(|| Vec::new());
    
    // Run on component mount
    {
        let counter = counter.clone();
        use_effect_with((), move |_| {
            let counter = counter.clone();
            spawn_local(async move {
                let mut __resp__: Option<postgrest::Builder> = None;
                DATA_BASE.with_borrow_mut(|__db_k__| {
                    __resp__ = Some(__db_k__.0
                        .from("blogpost")
                        .auth(__db_k__.1.clone())
                        .select("*"));
                });
                if let Some(builder) = __resp__ {
                    match builder.execute().await {
                        Ok(res) => {
                            if let Ok(text) = res.text().await {
                                counter.set(serde_json::from_str::<Vec<BlogPostData>>(&text).unwrap());
                            }
                        }
                        Err(_) => {counter.set(vec![BlogPostData{id: 0, title: "Error Loading Posts".to_string(), date: String::new(), preview: String::new(), content: String::new()}]);},
                    }
                }
            });
            || ()
        });
    }
    
    // let posts: Vec<BlogPostData> = counter.to_vec();
    let posts = get_blog_posts().unwrap();;

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
                                    <Link<Route> to={Route::BlogPost { id: post.id }}>
                                        {&post.title}
                                    </Link<Route>>
                                </h2>
                                <p class="blog-preview">{&post.preview}</p>
                                <Link<Route> 
                                    to={Route::BlogPost { id: post.id }} 
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