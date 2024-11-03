// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ExternalBuild {}

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

impl Component for ExternalBuild {
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
                data-icon={ "ExternalBuild" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M256 256v1536h256v128H128V128h1792v768h-128V256H256zm1649 1162l143 157v217h-128v-167l-101-112q-36 20-77 22l-78 89v168h-128v-216l72-82q-5-3-8-6t-8-8l-248-249-192 193v496h128v128H640v-128h128v-576q0-38 14-73t42-63l384-384q27-27 62-41t74-15q38 0 73 14t63 42l384 384q27 27 41 62t15 74q0 40-15 74zm-862-247q61 29 90 90l256-256q19-19 19-45t-19-45-45-19q-26 0-45 19l-256 256zm-147 749h128v-576q0-26-19-45t-45-19q-26 0-45 19t-19 45v576zm832-512q26 0 45-19t19-45q0-26-19-45l-256-256q-14 31-36 52t-47 46l249 248q19 19 45 19z" />
            </svg>
        }
    }
}
