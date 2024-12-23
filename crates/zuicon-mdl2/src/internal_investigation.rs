// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct InternalInvestigation {}

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

impl Component for InternalInvestigation {
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
                data-icon={ "InternalInvestigation" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M512 1408q0-83 40-154t111-117q-26-30-40-67t-15-78q0-46 17-87t48-71 71-48 88-18q46 0 87 17t71 48 48 72 18 87q0 40-14 77t-41 68q71 45 111 116t40 155h-128q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 62-15 74H512zm320-512q-40 0-68 28t-28 68q0 40 28 68t68 28q41 0 68-27t28-69q0-40-28-68t-68-28zm-448 768h579l-128 128H352q-40 0-68-28t-28-68V352q0-40 28-68t68-28h243L466 0h143l129 256h186L1052 0h143l-128 256h245q40 0 68 28t28 68v451q-35 19-67 41t-61 51V384H802q6 13 21 40t30 57 27 56 12 39q0 27-19 46t-45 19q-18 0-33-9t-24-26L659 384H384v1280zm1280-768q79 0 149 30t122 82 83 123 30 149q0 80-30 149t-82 122-123 83-149 30q-60 0-116-18t-106-54l-437 437q-19 19-45 19t-45-19-19-45q0-27 19-46l436-436q-34-49-52-105t-19-117q0-79 30-149t82-122 122-83 150-30zm0 640q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 53 20 99t55 82 81 55 100 20z" />
            </svg>
        }
    }
}
