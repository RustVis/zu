// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ProfileSearch {}

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

impl Component for ProfileSearch {
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
                data-icon={ "ProfileSearch" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1468 1139q-52 43-89 96-83-42-173-62t-184-21q-108 0-206 27t-184 76-154 119-119 155-76 185-27 206H128q0-146 43-281t124-247 193-196 254-129q-54-36-96-83t-72-102-46-116-16-126q0-106 40-199t110-162 163-110 199-41q106 0 199 40t162 110 110 163 41 199q0 65-16 126t-45 117-73 102-97 83q43 14 83 31t80 40zM640 640q0 80 30 149t82 122 122 83 150 30q79 0 149-30t122-82 83-122 30-150q0-79-30-149t-82-122-123-83-149-30q-80 0-149 30t-122 82-83 123-30 149zm1090 511q66 0 125 25t102 69 69 102 26 125q0 66-25 124t-69 102-103 69-125 26q-97 0-177-54l-292 292q-19 19-45 19t-45-19-19-45q0-26 19-45l292-292q-54-80-54-177 0-66 25-124t69-102 102-69 125-26zm0 514q40 0 75-15t61-41 42-62 16-75q0-40-15-75t-42-61-61-42-76-15q-40 0-75 15t-61 42-42 61-15 75q0 40 15 75t41 61 62 42 75 15z" />
            </svg>
        }
    }
}
