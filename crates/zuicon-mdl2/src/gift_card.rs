// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct GiftCard {}

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

impl Component for GiftCard {
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
                data-icon={ "GiftCard" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1936 256q23 0 43 9t36 24 24 35 9 44v1184q0 23-9 43t-24 36-35 24-44 9H112q-23 0-43-9t-36-24-24-35-9-44V368q0-23 9-43t24-36 35-24 44-9h1824zm-16 128H640v139q33-11 64-11 40 0 75 15t61 41 41 61 15 75q0 31-11 64h1035V384zM384 704q0 26 19 45t45 19h64v-64q0-26-19-45t-45-19q-26 0-45 19t-19 45zm320 64q26 0 45-19t19-45q0-26-19-45t-45-19q-26 0-45 19t-19 45v64h64zM128 384v384h139q-11-33-11-64 0-40 15-75t41-61 61-41 75-15q31 0 64 11V384H128zm0 1152h384V987l-147 146-90-90 146-147H128v640zm1792 0V896H731l146 147-90 90-147-146v549h1280z" />
            </svg>
        }
    }
}
