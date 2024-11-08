// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Spelling {}

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

impl Component for Spelling {
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
                data-icon={ "Spelling" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M547 788q-29 51-71 79t-103 29q-40 0-73-11t-58-34-37-54-13-74q0-45 13-78t38-57 59-38 77-21l170-25q0-30-5-57t-19-48-36-33-57-13q-53 0-99 19t-86 54v-98q16-12 40-22t52-17 54-12 47-5q54 0 92 15t62 45 35 70 11 92v387h-91v-93h-2zm-153 27q37 0 65-14t49-39 30-56 11-66v-60l-137 21q-28 5-51 11t-40 18-26 33-10 53q0 48 32 73t77 26zm465-19v85h-91V0h91v391h2q33-59 87-89t121-30q63 0 107 25t72 67 41 94 13 108q0 61-14 120t-46 105-80 76-119 29q-57 0-105-25t-77-75h-2zm0-241v83q0 39 12 71t36 56 55 37 71 13q51 0 85-23t54-60 28-80 9-87q0-39-8-77t-28-68-51-48-76-19q-45 0-80 15t-59 43-36 64-12 80zm840-202q-50 0-87 20t-62 53-37 76-12 88q0 45 11 86t35 72 60 49 87 18q38 0 73-14t66-38v91q-35 23-74 32t-81 10q-65 0-115-23t-84-64-53-95-18-116q0-69 19-128t56-104 92-69 128-25q34 0 67 7t64 23v97q-30-22-64-34t-71-12zM895 1829l724-722 90 90-814 814-428-430 90-90 338 338z" />
            </svg>
        }
    }
}
