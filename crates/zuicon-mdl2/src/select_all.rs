// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SelectAll {}

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

impl Component for SelectAll {
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
                data-icon={ "SelectAll" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M0 128V0h128v128H0zm0 256V256h128v128H0zm0 256V512h128v128H0zm0 256V768h128v128H0zm0 256v-128h128v128H0zm0 256v-128h128v128H0zm0 256v-128h128v128H0zm0 256v-128h128v128H0zM1792 384V256h128v128h-128zm0 256V512h128v128h-128zm0 256V768h128v128h-128zm0 256v-128h128v128h-128zm0 256v-128h128v128h-128zm0 256v-128h128v128h-128zm0 256v-128h128v128h-128zM384 0v128H256V0h128zm256 0v128H512V0h128zm256 0v128H768V0h128zm256 0v128h-128V0h128zm256 0v128h-128V0h128zm128 128V0h128v128h-128zM256 1920v-128h128v128H256zm256 0v-128h128v128H512zm256 0v-128h128v128H768zm256 0v-128h128v128h-128zm256 0v-128h128v128h-128zm256 0v-128h128v128h-128zM1792 0h128v128h-128V0zm-256 384v1152H384V384h1152zm-128 512V512h-384v384h384zM896 512H512v384h384V512zm-384 512v384h384v-384H512zm512 384h384v-384h-384v384z" />
            </svg>
        }
    }
}
