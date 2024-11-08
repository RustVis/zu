// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Variable2 {}

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

impl Component for Variable2 {
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
                data-icon={ "Variable2" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M139 960q0 223 59 430t165 402h-97q-62-91-111-191t-83-206-53-216T0 959q0-230 71-435t195-396h97q-55 108-96 205t-70 195-43 204-15 228zm1485-478q0 48-25 72t-73 24q-17 0-34-5t-33-11-33-11-34-5q-21 0-45 16t-49 42-50 58-46 64-38 60-26 45l101 426q3 13 8 35t14 45 21 39 30 17q23 0 50-20t53-48 45-58 29-50l43 21q-12 23-32 54t-46 64-54 65-61 57-61 42-59 16q-39 0-66-23t-45-55q-8-14-18-45t-20-71-22-85-20-86-16-76-11-54q-26 48-62 108t-78 122-86 115-85 88q-32 26-68 44t-78 18q-24 0-45-7t-38-22-27-35-10-45q0-42 26-69t68-27q22 0 41 11t35 23 31 24 27 11q18 0 47-28t63-72 69-97 66-103 52-90 32-59q-10-38-20-75t-19-75q-8-32-17-72t-22-81-33-75-47-55q-31-22-64-28t-71-6q-33 0-66 3v-44l332-59q40 43 65 85t42 85 29 91 28 102q32-48 64-98t69-95 80-84 98-66q18-9 38-14t40-6q22 0 43 6t38 18 26 30 10 44zm424 477q0 110-18 220t-53 215-84 206-111 192h-97q106-195 165-402t59-430q0-146-25-290t-78-282q-26-67-57-131t-64-129h97q124 191 195 396t71 435z" />
            </svg>
        }
    }
}
