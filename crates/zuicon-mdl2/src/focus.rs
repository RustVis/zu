// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Focus {}

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

impl Component for Focus {
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
                data-icon={ "Focus" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 120q125 0 240 32t216 91 183 141 141 183 91 216 33 241q0 125-32 240t-91 216-141 183-183 141-216 91-241 33q-125 0-240-32t-216-91-183-141-141-183-91-216-33-241q0-125 32-240t91-216 141-183 183-141 216-91 241-33zm0 1680q107 0 206-28t185-78 157-121 121-157 79-186 28-206q0-107-28-206t-78-185-121-157-157-121-186-79-206-28q-107 0-206 28t-185 78-157 121-121 157-79 186-28 206q0 107 28 206t78 185 121 157 157 121 186 79 206 28zm280-1160q21 0 40 8t33 22 23 34 8 40v560q0 21-8 40t-22 33-34 23-40 8H744q-21 0-40-8t-33-22-23-34-8-40V744q0-21 8-40t22-33 34-23 40-8h560zm-24 128H768v512h512V768z" />
            </svg>
        }
    }
}
