// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Money {}

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

impl Component for Money {
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
                data-icon={ "Money" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 384v1280H128v-256H0V256h1792v128h256zm-512 0q0 27 10 50t27 40 41 28 50 10V384h-128zM128 512q27 0 50-10t40-27 28-41 10-50H128v128zm0 512q53 0 99 20t82 55 55 81 20 100h1024q0-53 20-99t55-82 81-55 100-20V640q-53 0-99-20t-82-55-55-81-20-100H384q0 53-20 99t-55 82-81 55-100 20v384zm1536 128q-27 0-50 10t-40 27-28 41-10 50h128v-128zM128 1280h128q0-27-10-50t-27-40-41-28-50-10v128zm1792-768h-128v896H256v128h1664V512zM448 896q-26 0-45-19t-19-45q0-26 19-45t45-19q26 0 45 19t19 45q0 26-19 45t-45 19zm896 0q-26 0-45-19t-19-45q0-26 19-45t45-19q26 0 45 19t19 45q0 26-19 45t-45 19zm-448 256q-53 0-99-20t-82-55-55-81-20-100V768q0-53 20-99t55-82 81-55 100-20q53 0 99 20t82 55 55 81 20 100v128q0 53-20 99t-55 82-81 55-100 20zM768 896q0 27 10 50t27 40 41 28 50 10q27 0 50-10t40-27 28-41 10-50V768q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50v128z" />
            </svg>
        }
    }
}
