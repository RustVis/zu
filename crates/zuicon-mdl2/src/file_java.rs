// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct FileJava {}

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

impl Component for FileJava {
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
                data-icon={ "FileJAVA" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M256 1280H128V0h1115l549 549v731h-128V640h-512V128H256v1152zm1024-768h293l-293-293v293zM104 1957q34 0 54-15t32-39 15-52 4-56v-387h104v404q0 48-11 91t-37 75-63 51-91 19q-18 0-36-1t-35-9v-98q14 9 30 13t34 4zm621-549l234 630H844l-56-161H540l-54 161H372l235-630h118zm35 384l-87-251q-3-10-5-19t-5-20q-3 22-9 39l-86 251h192zm728-384l-226 630h-116l-223-630h115l155 478q5 13 7 26t6 27q2-14 5-27t8-27l159-477h110zm552 630h-115l-56-161h-248l-54 161h-115l236-630h118l234 630zm-199-246l-88-251q-3-9-5-19t-5-18q-2 9-3 19t-5 18l-87 251h193z" />
            </svg>
        }
    }
}
