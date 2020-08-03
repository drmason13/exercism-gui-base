#![allow(clippy::wildcard_imports)]

use seed::virtual_dom::IntoNodes;
use seed::virtual_dom::update_el::UpdateEl;
use seed::{prelude::*, *};

use std::marker::PhantomData;

// TODO: Styling, light theme and dark theme

#[derive(Default)]
pub struct Footer<Ms: 'static> {
    exercise: &'static str,
    description: &'static str,
    _marker: PhantomData<Ms>,
}

// It allows us to use `Footer` directly in element macros without calling `view` explicitly.
// E.g. `div![Footer::new("Footer Description")]`
impl<Ms> UpdateEl<Ms> for Footer<Ms> {
    fn update_el(self, el: &mut El<Ms>) {
        self.view().update_el(el)
    }
}

impl<Ms> IntoNodes<Ms> for Footer<Ms> {
    fn into_nodes(self) -> Vec<Node<Ms>> {
        vec![self.view()]
    }
}

impl<Ms> Footer<Ms> {
    pub fn new(exercise: &'static str, description: &'static str) -> Self {
        Footer {
            exercise,
            description,
            _marker: PhantomData,
        }
    }
    
    fn view(self) -> Node<Ms> {
        footer![
            C!["info"],
            p![self.description],
            p![
                "Created by ",
                a![
                    attrs!{At::Href => "https://github.com/drmason13", At::Target => "_blank", At::Rel => "noopener noreferrer"},
                    "David Mason"
                ]
            ],
            p![
                "Using ",
                a![
                    attrs!{At::Href => "https://seed-rs.org/", At::Target => "_blank", At::Rel => "noopener noreferrer"},
                    "Seed"
                ]
            ],
            p![
                "Inspired by the exercise of the same name from the ",
                a![
                    attrs!{At::Href => format!("https://exercism.io/tracks/rust/exercises/{}", self.exercise), At::Target => "_blank", At::Rel => "noopener noreferrer"},
                    "Exercism Rust track"
                ]
            ]
        ]
    }
}