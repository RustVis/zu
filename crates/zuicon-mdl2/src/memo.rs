// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Memo {}

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

impl Component for Memo {
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
                data-icon={ "Memo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1600 512q93 0 174 35t142 96 96 142 36 175q0 93-35 174t-96 142-142 96-175 36H448q-93 0-174-35t-142-96-96-142T0 960q0-93 35-174t96-142 142-96 175-36q93 0 174 35t142 96 96 142 36 175q0 21-4 50t-13 61-20 64-27 61-33 51-38 33h526q-20-11-38-33t-33-50-27-61-20-64-12-61-5-51q0-93 35-174t96-142 142-96 175-36zM128 960q0 66 25 124t69 101 102 69 124 26q66 0 124-25t101-69 69-102 26-124q0-66-25-124t-69-101-102-69-124-26q-66 0-124 25t-101 69-69 102-26 124zm1472 320q66 0 124-25t101-69 69-102 26-124q0-66-25-124t-69-101-102-69-124-26q-66 0-124 25t-101 69-69 102-26 124q0 66 25 124t69 101 102 69 124 26z" />
            </svg>
        }
    }
}
