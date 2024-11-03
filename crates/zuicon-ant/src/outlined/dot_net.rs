// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct DotNet {}

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

impl Component for DotNet {
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
                data-icon={ "dot-net" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <g fill-opacity=".88" transform="translate(64 362)"><path d="M37.283 300c-10.652 0-19.53-3.297-26.63-9.89C3.55 283.514 0 275.41 0 265.793c0-9.891 3.653-17.996 10.96-24.315 7.306-6.32 16.422-9.479 27.347-9.479 11.062 0 20.11 3.194 27.143 9.582C72.483 247.97 76 256.04 76 265.794c0 10.028-3.585 18.236-10.755 24.624-7.17 6.388-16.49 9.582-27.962 9.582M394 295h-66.969L205.624 109.648c-7.121-10.835-12.052-18.995-14.79-24.48h-.822c1.095 10.422 1.643 26.331 1.643 47.726V295H129V0h71.488l116.888 179.592c5.341 8.092 10.271 16.115 14.79 24.07h.822c-1.095-6.858-1.643-20.367-1.643-40.527V0H394z"/><path d="M638 295 461 295 461 0 631.20814 0 631.20814 54.1039052 527.477907 54.1039052 527.477907 119.728033 624.004651 119.728033 624.004651 173.62622 527.477907 173.62622 527.477907 241.101813 638 241.101813z"/><path d="M896 54.1039052 812.047368 54.1039052 812.047368 295 745.542105 295 745.542105 54.1039052 662 54.1039052 662 0 896 0z"/></g>
            </svg>
        }
    }
}
