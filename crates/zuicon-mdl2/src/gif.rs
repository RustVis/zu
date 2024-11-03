// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Gif {}

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

impl Component for Gif {
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
                data-icon={ "GIF" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1856 128q39 0 74 15t61 41 42 62 15 74v1280q0 39-15 74t-41 61-62 42-74 15H192q-39 0-74-15t-61-41-42-62-15-74V320q0-39 15-74t41-61 62-42 74-15h1664zm64 1472V320q0-26-19-45t-45-19H192q-26 0-45 19t-19 45v1280q0 26 19 45t45 19h1664q26 0 45-19t19-45zm-626-326V646h338v89h-234v188h216v88h-216v263h-104zm-656-254v-87h236v298q-51 27-106 40t-112 13q-71 0-128-23t-99-64-63-100-22-129q0-73 25-134t69-105 106-68 134-25q45 0 89 6t87 24v108q-38-26-82-37t-90-12q-52 0-94 18t-72 50-46 75-16 95q0 48 12 90t38 74 64 50 90 18q29 0 57-5t55-20v-150H638zm396 260V653h108v627h-108z" />
            </svg>
        }
    }
}
