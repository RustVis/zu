// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PenWorkspace {}

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

impl Component for PenWorkspace {
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
                data-icon={ "PenWorkspace" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1747 290q14 8 23 23t9 32q0 8-2 15t-5 14l-707 1415q-9 19-28 28l-173 87q-32 16-69 16h-9q-4 0-10-1l-47 94q-8 16-23 25t-34 10q-26 0-45-19t-19-45q0-12 7-30t16-37 20-37 15-28q-26-40-26-87v-165q0-16 7-29l576-1152-65-32-237 474q-8 16-23 25t-34 10q-26 0-45-19t-19-45q0-13 7-29l239-478q16-32 43-50t63-19q35 0 66 17t62 32l71-142q8-17 23-26t34-9q13 0 22 4 12-24 23-47t26-43 36-30 53-12q32 0 61 15l94 47q32 16 50 42t19 64q0 34-15 63t-30 59zm-202-101l87 43 29-58-87-43-29 58zm84 185l-192-96-669 1337v150q0 11 8 19t19 8q4 0 16-5t29-13 35-17 36-19 30-16 19-10l669-1338zm163 394q53 0 99 20t82 55 55 81 20 100q0 53-20 99t-55 82-81 55-100 20h-288l64-128h224q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-26 0-45-19t-19-45q0-26 19-45t45-19zM128 1600q0 66 25 124t68 102 102 69 125 25h44q-5 15-8 31t-4 33q0 17 3 33t9 31h-44q-93 0-174-35t-142-96-96-142-36-175q0-93 35-174t96-142 142-96 175-36h224l-64 128H448q-66 0-124 25t-102 69-69 102-25 124z" />
            </svg>
        }
    }
}
