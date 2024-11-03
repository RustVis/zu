// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BusSolid {}

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

impl Component for BusSolid {
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
                data-icon={ "BusSolid" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1920 640v256h-128v1024q0 27-10 50t-27 40-41 28-50 10h-128q-27 0-50-10t-40-27-28-41-10-50H512q0 27-10 50t-27 40-41 28-50 10H256q-27 0-50-10t-40-27-28-41-10-50V896H0V640h128V320q0-40 15-75t41-61 61-41 75-15h1280q40 0 75 15t61 41 41 61 15 75v320h128zM256 1152h640V640H256v512zm256 320q0-26-19-45t-45-19q-26 0-45 19t-19 45v64h128v-64zm1152 192H256v128h1408v-128zm-128-128v-64q0-26-19-45t-45-19q-26 0-45 19t-19 45v64h128zm128-896h-640v512h640V640z" />
            </svg>
        }
    }
}
