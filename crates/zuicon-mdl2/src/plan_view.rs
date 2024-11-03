// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PlanView {}

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

impl Component for PlanView {
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
                data-icon={ "PlanView" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M128 256v384h1920v128H128v1152H0V128h2048v128H128zm128 640h128v128H256V896zm256 0h128v128H512V896zm256 0h128v128H768V896zm256 0h128v128h-128V896zm256 0h128v128h-128V896zm256 0h128v128h-128V896zm256 0h128v128h-128V896zM256 1408h128v128H256v-128zm256 0h128v128H512v-128zm256 0h128v128H768v-128zm256 0h128v128h-128v-128zm256 0h128v128h-128v-128zm256 0h128v128h-128v-128zm256 0h128v128h-128v-128zM256 1152h128v128H256v-128zm256 0h128v128H512v-128zm256 0h128v128H768v-128zm256 0h128v128h-128v-128zm256 0h128v128h-128v-128zm256 0h128v128h-128v-128zm256 0h128v128h-128v-128zM256 1664h128v128H256v-128zm256 0h128v128H512v-128zm256 0h128v128H768v-128zm256 0h128v128h-128v-128zm256 0h128v128h-128v-128z" />
            </svg>
        }
    }
}
