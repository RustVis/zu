// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PageData {}

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

impl Component for PageData {
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
                data-icon={ "PageData" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 1216v618q0 41-19 73t-52 55-74 40-83 27-84 14-72 5q-32 0-73-4t-83-14-84-26-73-40-51-56-20-74v-618q0-39 20-69t54-51 75-35 84-22 83-12 69-3q29 0 69 3t82 11 84 22 75 35 53 51 20 70zm-128 618v-470q-60 24-125 34t-130 10q-63 0-130-10t-127-34v470q0 8 6 13t12 11q19 17 49 29t64 19 67 11 58 3q26 0 59-3t66-10 63-19 50-30q5-5 11-10t7-14zm0-621q-12-16-46-28t-73-19-78-10-58-4q-19 0-58 3t-81 12-76 22-43 31q13 17 47 28t74 18 78 11 59 3q19 0 58-3t80-12 74-20 43-32zm-256-573h-512V128H256v1792h896v128H128V0h1115l549 549v347h-128V640zm-384-421v293h293l-293-293z" />
            </svg>
        }
    }
}
