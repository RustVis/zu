// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Js {}

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

impl Component for Js {
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
                data-icon={ "JS" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1920 128v1792H128V128h1792zm-119 119H247v1554h1554V247zm-308 1214q0-36-25-60t-65-46-83-41-84-49-64-67-26-98q0-55 25-94t65-63 88-36 96-11q19 0 41 1t45 3 44 8 40 14v122q-35-26-77-35t-85-10q-23 0-49 4t-50 15-38 30-15 48q0 26 10 44t30 33q26 20 56 33t60 27q34 16 68 38t61 49 45 62 17 77q0 60-25 99t-65 63-90 33-100 10q-18 0-45-3t-56-8-55-13-40-18v-127q17 15 40 27t50 22 52 13 50 5q25 0 52-3t50-15 37-31 15-52zm-726 203q-41 0-79-12t-71-36-56-56-37-72l103-32q16 43 51 71t84 29q38 0 61-18t36-47 17-62 5-63V908h123v477q0 56-14 107t-43 89-73 60-107 23z" />
            </svg>
        }
    }
}
