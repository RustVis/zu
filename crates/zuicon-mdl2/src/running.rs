// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Running {}

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

impl Component for Running {
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
                data-icon={ "Running" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1792 768q40 0 75 15t61 41 41 61 15 75q0 40-15 75t-41 61-61 41-75 15h-320q-26 0-45-19l-147-146-102 101 211 211q19 19 19 45v384q0 40-15 75t-41 61-61 41-75 15q-40 0-75-15t-61-41-41-61-15-75v-229l-128-128-147 146q-19 19-45 19H256q-40 0-75-15t-61-41-41-61-15-75q0-40 15-75t41-61 61-41 75-15h293l80-79q-53-23-85-71t-32-106V512q0-27 10-50t27-40 41-28 50-10h512q0-53 20-99t55-82 81-55 100-20q53 0 99 20t82 55 55 81 20 100q0 42-13 80t-36 71-56 56-73 37l141 140h165zm-384-512q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10q24 0 47-9t41-26 29-38 11-47q0-28-9-53t-25-43-41-29-53-11zm384 768q26 0 45-19t19-45q0-32-18-46t-46-18-63-2-68 4-61-1-45-20l-366-365H640v384q0 26 19 45t45 19q37 0 50-23t16-60-2-77-2-77 15-59 51-24h192q26 0 45 19t19 45q0 26-19 45t-45 19H896v192q0 26-19 45l-256 256q-19 19-45 19H256q-26 0-45 19t-19 45q0 26 19 45t45 19h421l174-173q19-19 45-19t45 19l192 192q19 19 19 45v256q0 26 19 45t45 19q26 0 45-19t19-45v-357l-237-238q-19-19-19-45t19-45l192-192q19-19 45-19t45 19l174 173h293z" />
            </svg>
        }
    }
}
