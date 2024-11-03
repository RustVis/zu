// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CalculatorDelta {}

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

impl Component for CalculatorDelta {
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
                data-icon={ "CalculatorDelta" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M896 768v128H769V768h127zm256-512v384H256V256h896zm-128 256V384H384v128h640zm-384 768v128H513v-128h127zm256-256v128H769v-128h127zm129 640v-128h76l-64 128h-12zm-512 0v-128h127v128H513zm383-384v128H769v-128h127zm-127 384v-128h127v128H769zm-513 0v-128h128v128H256zm896-896v128h-127V768h127zm0 512v128h-127v-128h127zm0-256v128h-127v-128h127zm-512 0v128H513v-128h127zM128 128v1664h845l-64 128H0V0h1408v922l-128 256V128H128zm512 640v128H513V768h127zm-256 0v128H256V768h128zm0 512v128H256v-128h128zm0-256v128H256v-128h128zm1152 0l512 1024H1024l512-1024zm0 286l-305 610h610l-305-610z" />
            </svg>
        }
    }
}
