// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Merge {}

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

impl Component for Merge {
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
                data-icon={ "merge" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M112 0c61.856 0 112 50.144 112 112 0 49.262-31.803 91.095-75.998 106.088L148 502.371l386.488-126.553.014-95.914C489.009 265.664 456 223.187 456 173c0-61.856 50.144-112 112-112s112 50.144 112 112c0 48.327-30.608 89.505-73.496 105.206l-.018 113.037c-.003 21.932-14.1 41.379-34.944 48.204L148 578.132l.002 27.78c43.64 14.805 75.197 55.78 75.983 104.236L224 712c0 61.856-50.144 112-112 112S0 773.856 0 712c0-49.262 31.804-91.096 75.999-106.088V218.088C31.804 203.096 0 161.262 0 112 0 50.144 50.144 0 112 0" transform="matrix(1 0 0 -1 172 924)"/>
            </svg>
        }
    }
}
