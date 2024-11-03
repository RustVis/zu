// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Weights {}

#[derive(Properties, Debug, Clone, PartialEq, Eq)]
pub struct Props {
    #[prop_or_default]
    pub class: Option<&'static str>,

    #[prop_or_default]
    pub width: Option<&'static str>,

    #[prop_or_default]
    pub height: Option<&'static str>,

    #[prop_or_default]
    pub color: Option<&'static str>,

    #[prop_or_default]
    pub fill: Option<&'static str>,

    #[prop_or_default]
    pub spin: bool,

    #[prop_or_default]
    pub rotate: i16,
}

impl Component for Weights {
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        // TODO(Shaohua): Do not generate style attribute if it is empty.
        let mut style = String::new();
        if props.rotate != 0 {
            style += &format!("transform: rotate({}deg);", props.rotate);
        }
        html! {
            <svg
                xmlns={ "http://www.w3.org/2000/svg" }
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("16") }
                height={ props.height.unwrap_or("16") }
                focusable={ "false" }
                data-icon={ "Weights" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1883 640l128 128-283 282-320-319-677 677 319 320-282 283-128-128-160 159-192-191q-10 9-20 21t-22 22-25 18-29 8q-26 0-45-19t-19-45q0-15 7-28t18-25 23-22 21-21L6 1568l159-160-128-128 283-282 320 319 677-677-319-320 282-283 128 128L1568 6l192 191q10-9 20-21t22-22 25-18 29-8q26 0 45 19t19 45q0 15-7 28t-18 25-23 22-21 21l191 192-159 160zM549 1792l-293-293-69 69 293 293 69-69zm321-64l-550-550-101 102 549 549 102-101zm629-1472l293 293 69-69-293-293-69 69zm330 512l-549-549-102 101 550 550 101-102z" />
            </svg>
        }
    }
}
