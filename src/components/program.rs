use yew::prelude::*;
use serde::{Deserialize, Serialize};

// ***************************************
// DATA STRUCTURES - PROGRAM
// ***************************************

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct QAItem {
    pub question: String,
    pub answer: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CoreTenet {
    pub title: String,
    pub description: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Belief {
    pub name: String,
    pub description: String,
}

pub fn get_qa_items() -> Vec<QAItem> {
    vec![
        QAItem {
            question: "What is Hardin Valley Reform?".to_string(),
            answer: "Hardin Valley Reform is a student organized movement dedicated to working within our community to advocate for Left-Progressive ideas and perspectives, to protest against the injustices of a corrupt system, and to give back and be charitable to our community through food drives and volunteering. We believe that getting involved in your community early is important for your growth personally and for the benefit of your community, city, state, and country.".to_string(),
        },
        QAItem {
            question: "Who can join Hardin Valley Reform?".to_string(),
            answer: "Anyone who is a student or youth in Hardin Valley can join. ".to_string(),
        },
        QAItem {
            question: "How can I get involved?".to_string(),
            answer: "You can get involved by joining the community via our discord server! We are constantly trying to organize ways to reach out to our community and spread our message.".to_string(),
        },
    ]
}

pub fn get_core_tenets() -> Vec<CoreTenet> {
    vec![
        CoreTenet {
            title: "Advocate".to_string(),
            description: "Active political engagement is key to making a change in our community, our state, and our country. We want to mobilize our community to be more politically active and informed.".to_string(),
        },
        CoreTenet {
            title: "Protest".to_string(),
            description: "The government needs to be held accountable for the blatant injustices and corruption that plague our system. Protesting, while it may be somewhat ineffectual, is necessary to build solidarity between people.".to_string(),
        },
        CoreTenet {
            title: "Charity".to_string(),
            description: "Poverty runs rampant in the \"richest country in the world,\" we must give back through volunteering, food drives, and other charitable activities.".to_string(),
        },
    ]
}

pub fn get_beliefs() -> Vec<Belief> {
    vec![
        Belief {
            name: "Strong Unions".to_string(),
            description: "Due to the inherent antagonisms between the workers and their bosses, there must be some way to balance power and ensure fair treatment for all workers. The working class should have a way to fight for their rights democratically.".to_string(),
        },
        Belief {
            name: "Nationalization of Industries".to_string(),
            description: "We believe that a centralized industry owned by the public is better than a private monopoly industry that solely prioritizes profit rather than the general social good.".to_string(),
        },
        Belief {
            name: "End The Eternal Economic Warfare".to_string(),
            description: "We believe that most wars that the U.S. has fought in the last century have been driven by economic interests rather than ideological or security concerns. These economic interests have resulted in the deaths of millions of people.".to_string(),
        },
        Belief {
            name: "International economic cooperation".to_string(),
            description: "We believe that international economic cooperation is essential for building a more just and peaceful world. By working together across borders, we can address global challenges like poverty, inequality, and climate change.".to_string(),
        },
        Belief {
            name: "Free college for advanced development of an advanced economy".to_string(),
            description: "We believe that free education is essential for building a more advanced and equitable economy. By removing financial barriers to higher education, we can ensure that all students have the opportunity to develop their skills and contribute to the advancement of our society.".to_string(),
        },
        Belief {
            name: "Universal healthcare".to_string(),
            description: "We believe that universal healthcare is essential for ensuring that all people have access to quality medical care regardless of their economic status.".to_string(),
        },
        Belief {
            name: "A True Democratic Government".to_string(),
            description: "A \"democratic\" government filled with corruptable politicians taking money from every corporation and foreign interest inevitably decays into a democracy that does not care about its people; it becomes a democracy of the dollar that folds into oligarchy.".to_string(),
        },
    ]
}

// ***************************************
// PAGES - PROGRAM
// ***************************************

#[function_component(Program)]
pub fn program() -> Html {
    let qa_items = get_qa_items();
    let tenets = get_core_tenets();
    let beliefs = get_beliefs();

    html! {
        <div class="program-page">
            <div class="container">
                <header class="page-header">
                    <h1>{"Our Program"}</h1>
                    <p class="page-subtitle">
                        {"Understanding the principles and values that guide Hardin Valley Reform"}
                    </p>
                </header>

                <section class="qa-section">
                    <h2>{"Frequently Asked Questions"}</h2>
                    <div class="qa-list">
                        {qa_items.iter().map(|qa| html! {
                            <div class="qa-item">
                                <h3 class="question">{&qa.question}</h3>
                                <p class="answer">{&qa.answer}</p>
                            </div>
                        }).collect::<Html>()}
                    </div>
                </section>

                <section class="tenets-section">
                    <h2>{"Three Core Tenets"}</h2>
                    <div class="tenets-grid">
                        {tenets.iter().enumerate().map(|(i, tenet)| html! {
                            <div class="tenet-card">
                                <div class="tenet-number">{i + 1}</div>
                                <h3>{&tenet.title}</h3>
                                <p>{&tenet.description}</p>
                            </div>
                        }).collect::<Html>()}
                    </div>
                </section>

                <section class="beliefs-section">
                    <h2>{"What we believe in"}</h2>
                    <div class="beliefs-grid">
                        {beliefs.iter().enumerate().map(|(i, belief)| html! {
                            <div class="belief-card">
                                <div class="belief-number">{i + 1}</div>
                                <h3>{&belief.name}</h3>
                                <p>{&belief.description}</p>
                            </div>
                        }).collect::<Html>()}
                    </div>
                </section>
            </div>
        </div>
    }
}