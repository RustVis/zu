// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SeeDo {}

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

impl Component for SeeDo {
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
                data-icon={ "SeeDo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1408 0q40 0 78 12t70 36 56 54 38 70l363 1088q35 103 35 212 0 93-35 174t-96 142-142 96-175 36q-89 0-171-34t-146-97q-63-63-97-145t-34-172v-448H896v448q0 89-34 171t-97 146q-63 63-145 97t-172 34q-93 0-174-35t-142-96-96-142-36-175q0-109 35-212 5-14 23-68t44-134 59-176 66-195 64-192 55-166 38-116 15-44q15-37 39-68t55-54 69-34 78-13q74 0 121 24t76 64 42 93 16 110 3 114-2 107h256q0-49-1-106t1-115 17-109 41-93 76-65 122-24zM448 1792q66 0 124-25t101-69 69-102 26-124q0-66-25-124t-69-101-102-69-124-26q-66 0-124 25t-101 69-69 102-26 124q0 66 25 124t69 101 102 69 124 26zM768 256q0-27-10-50t-27-40-41-28-50-10q-39 0-73 24t-48 62l-289 866q51-27 106-41t112-15q33 0 80 9t95 27 87 43 58 56V256zm384 640V640H896v256h256zm128 263q18-31 57-56t87-42 95-27 81-10q57 0 112 14t106 42l-289-866q-13-38-47-62t-74-24q-27 0-50 10t-40 27-28 41-10 50v903zm320 633q66 0 124-25t101-69 69-102 26-124q0-66-25-124t-69-101-102-69-124-26q-66 0-124 25t-101 69-69 102-26 124q0 66 25 124t69 101 102 69 124 26z" />
            </svg>
        }
    }
}
