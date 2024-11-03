// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct XamarinLogo {}

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

impl Component for XamarinLogo {
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
                data-icon={ "XamarinLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 1024q0 20-4 40t-16 38l-434 740q-21 36-58 57t-79 21H591q-42 0-79-21t-58-57L20 1102q-11-17-15-37t-5-41q0-20 4-40t16-38l434-740q21-36 58-57t79-21h866q42 0 79 21t58 57l434 740q11 17 15 37t5 41zm-603 490q8 0 13-6t6-15q0-3-17-35t-42-79-57-102-58-104-49-85-26-46q5-10 26-46t48-86 59-104 56-102 43-79 17-36q0-8-5-14t-14-6h-149q-12 0-17 10-4 7-17 30t-31 56-41 73-46 81-44 79-39 69-27 50-10 22q0-2-10-21t-27-50-38-69-45-80-45-81-41-73-32-56-18-30q-5-10-16-10H599q-6 0-10 7t-5 13q0 3 10 22t26 49 37 68 43 78 44 79 40 71 32 55 17 31q-6 10-26 46t-48 85-59 104-56 102-43 79-17 35q0 8 5 14t14 7h149q10 0 16-11 5-10 26-47t50-87 60-106 58-104 44-80 18-36q0 3 17 35t44 80 58 104 60 107 49 87 27 47q6 11 17 11h149z" />
            </svg>
        }
    }
}
