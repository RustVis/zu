// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CiCircle {}

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

impl Component for CiCircle {
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
                data-icon={ "ci-circle" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm-63.6 656c-103 0-162.4-68.6-162.4-182.6v-49C286 373.5 345.4 304 448.3 304c88.3 0 152.3 56.9 152.3 138.1 0 2.4-2 4.4-4.4 4.4h-52.6c-4.2 0-7.6-3.2-8-7.4-4-46.1-37.6-77.6-87-77.6-61.1 0-95.6 45.4-95.6 126.9v49.3c0 80.3 34.5 125.1 95.6 125.1 49.3 0 82.8-29.5 87-72.4.4-4.1 3.8-7.3 8-7.3h52.7c2.4 0 4.4 2 4.4 4.4 0 77.4-64.3 132.5-152.3 132.5zM738 704.1c0 4.4-3.6 8-8 8h-50.4c-4.4 0-8-3.6-8-8V319.9c0-4.4 3.6-8 8-8H730c4.4 0 8 3.6 8 8v384.2z"/>
            </svg>
        }
    }
}
