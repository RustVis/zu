// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Compare {}

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

impl Component for Compare {
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
                data-icon={ "Compare" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1603 256l214 640h103v128h-22q-20 57-56 104t-84 81-104 52-118 19q-61 0-117-18t-104-52-84-81-57-105h-22V896h103l214-640h-445v1410q167 11 316 75t273 179h179v128H128v-128h179q123-114 272-178t317-76V256H451l214 640h103v128h-22q-20 57-56 104t-84 81-104 52-118 19q-61 0-117-18t-104-52-84-81-57-105H0V896h103l214-640H0V128h896V0h128v128h896v128h-317zM384 458L238 896h292L384 458zm0 694q69 0 128-34t94-94H162q35 60 94 94t128 34zm1020 768q-100-63-213-95t-231-33q-118 0-231 32t-213 96h888zm132-1462l-146 438h292l-146-438zm0 694q69 0 128-34t94-94h-444q35 60 94 94t128 34z" />
            </svg>
        }
    }
}
