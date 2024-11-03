// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Truck {}

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

impl Component for Truck {
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
                data-icon={ "truck" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M608 192c17.673 0 32 14.327 32 32v160h174.815a32 32 0 0 1 26.676 14.327l113.186 170.846A32 32 0 0 1 960 586.846V672c0 17.673-14.327 32-32 32h-96c0 70.692-57.308 128-128 128-70.692 0-128-57.308-128-128H384c0 70.692-57.308 128-128 128-70.692 0-128-57.308-128-128H96c-17.673 0-32-14.327-32-32V224c0-17.673 14.327-32 32-32zM256 640c-35.346 0-64 28.654-64 64 0 35.346 28.654 64 64 64l1.058-.009C291.916 767.426 320 738.993 320 704c0-35.346-28.654-64-64-64m448 0c-35.346 0-64 28.654-64 64 0 35.346 28.654 64 64 64l1.058-.009C739.916 767.426 768 738.993 768 704c0-35.346-28.654-64-64-64m93.629-192H640v145.124C658.829 582.234 680.687 576 704 576c47.378 0 88.745 25.741 110.876 64H896v-43.516zM500 448H332c-6.627 0-12 5.373-12 12v40c0 6.627 5.373 12 12 12h168c6.627 0 12-5.373 12-12v-40c0-6.627-5.373-12-12-12M308 320H204c-6.627 0-12 5.373-12 12v40c0 6.627 5.373 12 12 12h104c6.627 0 12-5.373 12-12v-40c0-6.627-5.373-12-12-12"/>
            </svg>
        }
    }
}