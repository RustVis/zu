// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CampaignTemplate {}

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

impl Component for CampaignTemplate {
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
                data-icon={ "CampaignTemplate" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 940v916l-517-74q-9 57-38 105t-71 85-96 56-110 20q-49 0-94-13t-83-39-68-62-50-81q-15-35-19-66t-4-66v-29q-30-5-67-11t-77-13-76-10-66-6h-14q-22 0-43 1t-43 1v-512q28 0 55 1t55 1h9q4 0 9-2l1408-202zm-644 824l-378-54q-2 5-2 9t0 9q0 40 15 75t41 61 61 41 75 15q34 0 65-11t57-33 42-49 24-63zm516-56v-620L640 1270v256l1280 182zM128 0v128H0V0h128zm0 256v128H0V256h128zm0 256v128H0V512h128zm0 256v128H0V768h128zm0 256v128H0v-128h128zm0 256v128H0v-128h128zm0 256v128H0v-128h128zM0 1792h128v128H0v-128zm128 256v-128h128v128H128zM384 0v128H256V0h128zm0 2048v-128h128v128H384zm256 0v-128h128v128H640zM640 0v128H512V0h128zm256 0v128H768V0h128zm256 0v128h-128V0h128zm256 0v128h-128V0h128zm128 256h-128V128h128v128zm0 128v128h-128V384h128zm-128 384V640h128v128h-128z" />
            </svg>
        }
    }
}
