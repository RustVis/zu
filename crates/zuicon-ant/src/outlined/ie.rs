// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Ie {}

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

impl Component for Ie {
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
                data-icon={ "ie" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M852.6 367.6c16.3-36.9 32.1-90.7 32.1-131.8 0-109.1-119.5-147.6-314.5-57.9-161.4-10.8-316.8 110.5-355.6 279.7 46.3-52.3 117.4-123.4 183-151.7C316.1 378.3 246.7 470 194 565.6c-31.1 56.9-66 148.8-66 217.5 0 147.9 139.3 129.8 270.4 63 47.1 23.1 99.8 23.4 152.5 23.4 145.7 0 276.4-81.4 325.2-219H694.9c-78.8 132.9-295.2 79.5-295.2-71.2h493.2c9.6-65.4-2.5-143.6-40.3-211.7zM224.8 648.3c26.6 76.7 80.6 143.8 150.4 185-133.1 73.4-259.9 43.6-150.4-185zm174-163.3c3-82.7 75.4-142.3 156-142.3 80.1 0 153 59.6 156 142.3h-312zm276.8-281.4c32.1-15.4 72.8-33 108.8-33 47.1 0 81.4 32.6 81.4 80.6 0 30-11.1 73.5-21.9 101.8-39.3-63.5-98.9-122.4-168.3-149.4z"/>
            </svg>
        }
    }
}
