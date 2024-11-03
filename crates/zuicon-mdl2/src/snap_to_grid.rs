// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SnapToGrid {}

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

impl Component for SnapToGrid {
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
                data-icon={ "SnapToGrid" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M384 0v128H256V0h128zM128 256v128H0V256h128zm0-256v128H0V0h128zm0 512v128H0V512h128zm768 256V640h128v128H896zm0-768v128H768V0h128zm128 384v128H896V384h128zM1408 0v128h-128V0h128zm256 0v128h-128V0h128zM0 1920v-128h128v128H0zM1024 256H896V128h128v128zM1152 0v128h-128V0h128zM640 0v128H512V0h128zm1152 640V512h128v128h-128zM512 1920v-128h128v128H512zM768 896v128H640V896h128zm1024 0V768h128v1152H768v-128h128V896h896zm0 896v-768h-768v768h768zm0-1792h128v128h-128V0zm0 384V256h128v128h-128zM0 1664v-128h128v128H0zm512-768v128H384V896h128zM0 1152v-128h128v128H0zm0 256v-128h128v128H0zm0-640h128v128H0V768zm256 128v128H128V896h128zm0 1024v-128h128v128H256z" />
            </svg>
        }
    }
}
