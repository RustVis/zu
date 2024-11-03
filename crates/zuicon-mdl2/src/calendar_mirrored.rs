// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CalendarMirrored {}

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

impl Component for CalendarMirrored {
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
                data-icon={ "CalendarMirrored" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1280 768h-128v128h128V768zm-384 768H768v128h128v-128zM512 768H384v128h128V768zm384 0H768v128h128V768zm384 256h-128v128h128v-128zm384 0h-128v128h128v-128zm-1152 0H384v128h128v-128zm384 0H768v128h128v-128zm384 256h-128v128h128v-128zm384 0h-128v128h128v-128zm-1152 0H384v128h128v-128zm384 0H768v128h128v-128zm384 256h-128v128h128v-128zm384 0h-128v128h128v-128zM0 128v1792h2048V128h-384V0h-128v128H512V0H384v128H0zm1920 128v256H128V256h256v128h128V256h1024v128h128V256h256zM128 1792V640h1792v1152H128z" />
            </svg>
        }
    }
}
