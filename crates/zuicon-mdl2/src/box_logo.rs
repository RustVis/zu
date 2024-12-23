// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BoxLogo {}

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

impl Component for BoxLogo {
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
                data-icon={ "BoxLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1057 644q79 0 148 30t122 82 82 121 30 149q0 79-30 148t-82 122-121 82-149 30q-53 0-103-14t-93-40-80-64-61-85q-25 47-61 84t-80 64-94 41-103 14q-77 0-146-29t-122-81-83-119-31-147V398q0-31 23-53t53-22q30 0 53 22t24 53v322q50-37 108-56t121-20q53 0 103 14t94 40 80 64 61 85q25-47 61-84t79-64 94-41 103-14zm-675 611q47 0 89-18t73-49 49-73 18-89q0-48-18-89t-49-73-72-49-90-18q-48 0-89 18t-73 49-49 72-18 90q0 47 18 89t49 73 73 49 89 18zm675 0q47 0 89-18t73-49 49-73 18-89q0-48-18-89t-49-73-72-49-90-18q-47 0-89 18t-73 49-49 73-18 89q0 47 18 89t49 73 73 49 89 18zm975 25q16 19 16 45 0 34-25 55t-59 21q-18 0-35-7t-30-22l-178-219-179 219q-12 14-29 21t-36 8q-33 0-58-21t-26-55q0-24 16-45l207-255-207-255q-16-21-16-45 0-17 7-31t18-24 27-15 31-6q18 0 36 7t30 22l179 220 178-220q11-14 30-21t37-8q16 0 30 5t26 16 19 25 7 30q0 24-16 45l-208 255 208 255z" />
            </svg>
        }
    }
}
