// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ContentSettings {}

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

impl Component for ContentSettings {
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
                data-icon={ "ContentSettings" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 128v1064q-28-31-60-57t-68-50V640H128v1024h868q3 33 10 65t18 63H0V128h2048zm-128 128H128v256h1792V256zM256 1536V768h640v768H256zm128-128h384V896H384v512zm1024-512h-384V768h384v128zm-384 130h384q-122 41-216 126h-168v-126zm1014 462l-124 51q6 30 6 59 0 32-6 63l124 51-49 119-124-52q-17 25-38 47t-48 39l52 124-119 49-51-124q-30 6-61 6-15 0-30-3t-30-6l-52 127-119-49 52-125q-25-17-45-39t-38-47l-127 53-49-119 124-51q-6-30-6-59 0-31 6-63l-124-51 49-119 124 52q17-25 38-47t48-39l-52-124 119-49 51 124q30-6 59-6 31 0 63 6l51-124 119 49-52 124q25 17 47 38t39 48l124-52 49 119zm-438 303q40 0 75-15t61-41 41-61 15-74q0-39-15-74t-41-61-62-42-74-15q-39 0-74 15t-61 41-42 62-15 74q0 40 15 74t41 61 62 41 74 15z" />
            </svg>
        }
    }
}
