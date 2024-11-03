// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Uneditable2mirrored {}

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

impl Component for Uneditable2mirrored {
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
                data-icon={ "Uneditable2Mirrored" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 2048l-633-158-583-583-723 722-90-90L1939 19l90 90-722 723 583 583 158 633zm-505-258l329 82-82-329q-47 10-87 32t-73 55-55 73-32 87zm-327-867l-293 293 505 506q16-52 44-98t67-85 84-66 99-45l-506-505zM0 336q0-70 26-131T98 99t107-72T335 0q67 0 128 25t110 73l530 531-90 90-373-372-293 293 372 373-90 90L98 573q-48-48-73-109T0 336zm128 0q0 38 10 66t29 53 41 47 48 47l293-293q-25-25-47-48t-46-41-54-28-67-11q-43 0-80 16t-66 45-44 66-17 81z" />
            </svg>
        }
    }
}
