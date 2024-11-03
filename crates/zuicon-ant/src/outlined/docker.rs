// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Docker {}

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

impl Component for Docker {
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
                data-icon={ "docker" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M491.877 286.236h-92.612v-82.784h92.612zm0-286.236h-92.612v85.59h92.612zm109.45 203.452h-92.612v82.784h92.612zm-218.9-101.024h-92.612v84.187h92.612zm109.45 0h-92.612v84.187h92.612zm388.688 140.311c-19.645-14.03-67.354-18.24-102.434-11.225-4.21-33.674-23.855-63.14-57.532-89.8l-19.645-12.627-12.628 19.644c-25.258 39.287-32.274 103.83-5.613 145.924-12.63 7.015-36.484 15.434-67.354 15.434H3.56c-12.629 71.56 8.42 164.165 61.741 227.305C117.221 599.131 195.8 630 296.832 630c220.302 0 384.478-101.024 460.25-286.236 29.468 0 95.419 0 127.692-63.14 1.404-2.807 9.823-18.24 11.226-23.853zm-717.038-39.287H70.915v82.784h92.612zm109.45 0h-92.612v82.784h92.612zm109.45 0h-92.612v82.784h92.612zm-109.45-101.024h-92.612v84.187h92.612z" transform="translate(64 202)"/>
            </svg>
        }
    }
}
