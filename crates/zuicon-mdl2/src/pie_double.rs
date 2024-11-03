// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PieDouble {}

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

impl Component for PieDouble {
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
                data-icon={ "PieDouble" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 128q58 0 113 3t110 11 107 23 108 38q133 57 241 148t185 206 118 251 42 280q0 133-34 255t-96 230-150 194-195 150-229 97-256 34q-195 0-368-72t-311-209l-45-45 660-661V128zm64 1792q115 0 221-30t198-84 169-130 130-168 84-199 30-221q0-108-27-209t-76-191-119-165-155-132-184-90-207-43v857l-605 605q114 98 252 149t289 51zM896 896H0q0-124 32-238t90-214 140-181 181-140 214-91T896 0v896zM768 138q-121 21-226 76T353 353 215 541t-77 227h630V138zm96 886l-624 624q-118-128-179-289T0 1024h864zm-726 128q13 81 41 157t73 146l303-303H138z" />
            </svg>
        }
    }
}
