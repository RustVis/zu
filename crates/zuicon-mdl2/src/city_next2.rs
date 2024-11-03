// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CityNext2 {}

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

impl Component for CityNext2 {
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
                data-icon={ "CityNext2" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M512 1024H384V768h128v256zm256 0H640V768h128v256zm256 0H896V768h128v256zm-512 384H384v-256h128v256zm256 0H640v-256h128v256zm256 0H896v-256h128v256zm384-640q106 0 199 40t163 109 110 163 40 200v768h-128v-768q0-79-30-149t-82-122-123-83-149-30h-128v1152H768v-384H640v384H128V512h128V256h256V0h384v256h256v256h128v256h128zM640 256h128V128H640v128zM384 512h640V384H384v128zm768 128H256v1280h256v-384h384v384h256V640zm256 512h256v128h-256v-128zm0 256h256v128h-256v-128zm0 256h256v128h-256v-128zm0 256h256v128h-256v-128z" />
            </svg>
        }
    }
}
