// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BadgeSd {}

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

impl Component for BadgeSd {
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
                data-icon={ "badge-sd" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path fill-rule="evenodd" d="M15 4a1 1 0 0 0-1-1H2a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1zM0 4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2zm5.077 7.114c-1.524 0-2.263-.8-2.315-1.749h1.147c.079.466.527.809 1.234.809.739 0 1.13-.339 1.13-.83 0-.418-.3-.634-.923-.779l-.927-.215c-.932-.21-1.52-.747-1.52-1.657 0-1.098.918-1.815 2.24-1.815 1.371 0 2.162.77 2.22 1.692H6.238c-.075-.43-.466-.76-1.103-.76-.655 0-1.046.338-1.046.804 0 .36.294.598.821.712l.932.216c.971.22 1.613.685 1.613 1.691 0 1.117-.857 1.881-2.378 1.881M8.307 11V5.001h2.19c1.823 0 2.684 1.09 2.684 2.984 0 1.908-.874 3.015-2.685 3.015zm2.031-5.032h-.844v4.06h.844c1.116 0 1.622-.667 1.622-2.02 0-1.354-.51-2.04-1.622-2.04"/>
            </svg>
        }
    }
}
