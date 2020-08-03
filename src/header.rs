#![allow(clippy::wildcard_imports)]

use seed::virtual_dom::IntoNodes;
use seed::virtual_dom::update_el::UpdateEl;
use seed::{prelude::*, *};

use std::marker::PhantomData;

// TODO: Styling, light theme and dark theme

#[derive(Default)]
pub struct Header<Ms: 'static> {
    title: &'static str,
    _marker: PhantomData<Ms>,
}

// It allows us to use `Header` directly in element macros without calling `view` explicitly.
// E.g. `div![Header::new("My Title")]`
impl<Ms> UpdateEl<Ms> for Header<Ms> {
    fn update_el(self, el: &mut El<Ms>) {
        self.view().update_el(el)
    }
}

impl<Ms> IntoNodes<Ms> for Header<Ms> {
    fn into_nodes(self) -> Vec<Node<Ms>> {
        vec![self.view()]
    }
}

impl<Ms> Header<Ms> {
    pub fn new(title: &'static str) -> Self {
        Header {
            title,
            _marker: PhantomData,
        }
    }

    fn view(self) -> Node<Ms> {
        header![
            C!["header"],
            h1![self.title],
        ]
    }
}
