// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct WebcamFill {}

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

impl Component for WebcamFill {
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
                data-icon={ "webcam-fill" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M6.644 11.094a.5.5 0 0 1 .356-.15h2a.5.5 0 0 1 .356.15c.175.177.39.347.603.496a7 7 0 0 0 .752.456l.01.006h.003A.5.5 0 0 1 10.5 13h-5a.5.5 0 0 1-.224-.947l.002-.001.01-.006a4 4 0 0 0 .214-.116 8 8 0 0 0 .539-.34c.214-.15.428-.319.603-.496M7 6.5a1 1 0 1 1 2 0 1 1 0 0 1-2 0"/>
  <path d="M2 3a2 2 0 0 0-2 2v3a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2zm6 1.5a2 2 0 1 1 0 4 2 2 0 0 1 0-4M12.5 7a.5.5 0 1 1 0-1 .5.5 0 0 1 0 1"/>
            </svg>
        }
    }
}
