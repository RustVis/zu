// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Streaming {}

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

impl Component for Streaming {
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
                data-icon={ "Streaming" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 779q51 0 95 19t78 53 52 77 20 96q0 51-19 95t-53 78-77 52-96 20q-51 0-95-19t-78-53-52-77-20-96q0-51 19-95t53-78 77-52 96-20zm0 384q29 0 54-11t44-29 30-44 11-55q0-29-11-54t-29-44-44-30-55-11q-29 0-54 11t-44 29-30 44-11 55q0 29 11 54t29 44 44 30 55 11zm716-855q72 71 127 154t93 174 57 189 20 199q0 101-19 199t-58 189-93 174-127 154l-75-75q64-64 113-138t83-156 51-169 18-178q0-90-17-177t-51-170-83-156-114-138l75-75zM383 383q-64 64-113 138t-84 156-51 169-18 178q0 90 17 177t52 170 83 156 114 138l-75 75q-72-71-127-154t-93-174-57-189-20-199q0-101 19-199t58-189 93-174 127-154l75 75zm1086 196q89 90 136 204t48 241q0 126-47 240t-137 205l-75-75q74-74 113-169t40-201q0-105-39-200t-114-170l75-75zm-815 75q-74 74-113 169t-40 201q0 105 39 200t114 170l-75 75q-89-90-136-204t-48-241q0-126 47-240t137-205l75 75z" />
            </svg>
        }
    }
}
