use yew::prelude::*;
use serde::{Deserialize, Serialize};

// ***************************************
// DATA STRUCTURES - READING
// ***************************************

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub cover_url: String,
    pub description: String,
}

// TODO: update to retrieve from DB
pub fn get_books() -> Vec<Book> {
    vec![
        Book {
            title: "The Federalist Papers".to_string(),
            author: "Alexander Hamilton, James Madison, John Jay".to_string(),
            cover_url: "https://images.placeholders.dev/?width=300&height=450&text=Federalist%20Papers&bgColor=%23f7f6f3&textColor=%236b7280".to_string(),
            description: "Essential reading for understanding the foundations of American government and the principles of federalism, separation of powers, and republican governance.".to_string(),
        },
    ]
}

// ***************************************
// PAGES - READING
// ***************************************

#[function_component(Reading)]
pub fn reading() -> Html {
    let books = get_books();

    html! {
        <div class="reading-page">
            <div class="container">
                <header class="page-header">
                    <h1>{"Reading List"}</h1>
                    <p class="page-subtitle">
                        {"Books that inform our principles and worldview"}
                    </p>
                </header>

                <section class="why-read">
                    <h2>{"Why Read?"}</h2>
                    <div class="why-read-content">
                        <p>
                            {"Informed citizens are the backbone of any healthy democracy. Reading deepens our understanding of history, philosophy, economics, and the human condition. The books listed here have shaped our thinking on governance, community, and the principles that make societies flourish."}
                        </p>
                        <p>
                            {"We encourage everyone to engage with these ideas, challenge them, and form their own conclusions. An educated electorate makes better decisions, asks harder questions, and builds stronger communities."}
                        </p>
                    </div>
                </section>

                <section class="books-section">
                    <h2>{"Recommended Books"}</h2>
                    <div class="books-grid">
                        {books.iter().map(|book| html! {
                            <div class="book-card">
                                <div class="book-cover">
                                    <img src={book.cover_url.clone()} alt={format!("{} cover", book.title)} />
                                </div>
                                <div class="book-info">
                                    <h3 class="book-title">{&book.title}</h3>
                                    <p class="book-author">{&book.author}</p>
                                    <p class="book-description">{&book.description}</p>
                                </div>
                            </div>
                        }).collect::<Html>()}
                    </div>
                </section>
            </div>
        </div>
    }
}