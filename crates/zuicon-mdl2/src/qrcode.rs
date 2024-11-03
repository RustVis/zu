// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Qrcode {}

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

impl Component for Qrcode {
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
                data-icon={ "QRCode" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M384 640V384h256v256H384zM128 896V128h768v768H128zm128-640v512h512V256H256zm128 1408v-256h256v256H384zm-256 256v-768h768v768H128zm128-640v512h512v-512H256zm1408-896v256h-256V384h256zm-512-256h768v768h-768V128zm640 640V256h-512v512h512zm-640 768h128v128h-128v-128zm256-256h-128v128h-128v-256h256v128zm256-128v128h-128v-128h128zm-128 128v256h-256v-128h128v-128h128zm-256 384h256v128h-128v128h-256v-128h128v-128zm256 256v-128h256v128h-256zm384-128h-128v-256h128v256zm-128-512v-128h128v256h-128v128h-128v-256h128zm-256 384v-128h128v128h-128z" />
            </svg>
        }
    }
}
