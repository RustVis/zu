// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Translate {}

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

impl Component for Translate {
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
                data-icon={ "Translate" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M651 896l341 1024H853l-85-256H384l-85 256H160L501 896h150zm74 640l-149-448-149 448h298zm81-992q1-40 1-79t0-79v-58q0-29-1-58 0-5 2-11 8-2 13-2 42 1 83 2t84 1h275v-11q0-30-1-61t-8-60q69 0 137 6 6 1 12 3t7 10q0 6-3 12t-5 12q-2 5-3 15t-2 22-1 22 0 17v13h296q42 0 83-1t84-2q7 0 13 3 3 5 3 10-1 17-2 35t-1 36v58q0 36 1 72t2 73v7q0 4-3 6-8 2-13 2h-25q-20 0-41 1-19 0-34-1t-17-3q-2-8-2-24t-1-36q0-33 1-68t1-52H939v39q0 15 1 31t0 34q0 29-1 52t-3 26q-8 2-13 2H821q-11 0-13-3t-2-13zm422 609q24 0 36-7t13-34V922H958q-40 0-80 1t-80 2q-8 0-12-3-2-6-2-11v-96-7q0-4 3-7 6-2 11-2l80 2q40 1 80 1h319q-2-26-2-52t-6-53q20 2 39 3t40 4h3q1 0 4 1 35-25 67-53t64-57h-308q-42 0-83 1t-84 2q-8 0-12-3-2-6-2-11v-95q0-4 2-12 8-2 12-2 42 1 83 2t84 1h373q16 0 30-4t25-5q8 0 23 12t30 28 26 32 12 25q0 10-6 15t-15 10q-14 7-27 18t-25 21q-52 43-104 83t-110 78v11h321q40 0 79-1t79-2q7 0 13 3 2 8 2 13v96q0 4-2 12-8 2-13 2-40-1-79-2t-79-1h-321v33q0 52 1 104t3 104v4q0 24-9 48t-29 38q-14 10-39 15t-55 8-56 3-44 1h-19q-13 0-18-6-3-3-7-16t-6-18q-7-26-16-48t-24-46q34 4 68 5t68 2zm647 300l-83-82v165q0 53-20 99t-55 82-81 55-100 20h-165l82 83-90 90-237-237 237-237 90 90-82 83h165q27 0 50-10t40-27 28-41 10-50v-165l-83 82-90-90 237-237 237 237-90 90z" />
            </svg>
        }
    }
}
