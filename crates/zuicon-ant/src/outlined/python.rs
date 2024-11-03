// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Python {}

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

impl Component for Python {
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
                data-icon={ "python" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <g><path d="M443 678.5c0 15.74 12.76 28.5 28.5 28.5s28.5-12.76 28.5-28.5-12.76-28.5-28.5-28.5-28.5 12.76-28.5 28.5M300 121.5c0 15.74 12.76 28.5 28.5 28.5s28.5-12.76 28.5-28.5S344.24 93 328.5 93 300 105.76 300 121.5" transform="translate(112 112)"/><path d="M709.524 185.714h-95.238V90.476C614.286 40.571 573.714 0 523.81 0H276.19c-49.904 0-90.476 40.571-90.476 90.476v95.238H90.476C40.571 185.714 0 226.286 0 276.19v247.62c0 49.904 40.571 90.476 90.476 90.476h95.238v95.238c0 49.905 40.572 90.476 90.476 90.476h247.62c49.904 0 90.476-40.571 90.476-90.476v-95.238h95.238c49.905 0 90.476-40.572 90.476-90.476V276.19c0-49.904-40.571-90.476-90.476-90.476M90.476 557.143c-18.38 0-33.333-14.953-33.333-33.333V276.19c0-18.38 14.952-33.333 33.333-33.333h278.572c15.81 0 28.571-12.762 28.571-28.571 0-15.81-12.762-28.572-28.571-28.572h-126.19V90.476c0-18.38 14.952-33.333 33.332-33.333h247.62c18.38 0 33.333 14.952 33.333 33.333v256.476c0 13.524-10.953 24.477-24.476 24.477H267.333c-45.047 0-81.619 36.666-81.619 81.619v104.095zm652.381-33.333c0 18.38-14.952 33.333-33.333 33.333H430.952c-15.81 0-28.571 12.762-28.571 28.571 0 15.81 12.762 28.572 28.571 28.572h126.19v95.238c0 18.38-14.952 33.333-33.332 33.333H276.19c-18.38 0-33.333-14.952-33.333-33.333V453.048c0-13.524 10.953-24.477 24.476-24.477h265.334c45.047 0 81.619-36.666 81.619-81.619V242.857h95.238c18.38 0 33.333 14.953 33.333 33.333z" transform="translate(112 112)"/></g>
            </svg>
        }
    }
}
