// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Dribbble {}

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

impl Component for Dribbble {
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
                data-icon={ "dribbble" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M512 96C282.6 96 96 282.6 96 512s186.6 416 416 416 416-186.6 416-416S741.4 96 512 96zm275.1 191.8c49.5 60.5 79.5 137.5 80.2 221.4-11.7-2.5-129.2-26.3-247.4-11.4-2.5-6.1-5-12.2-7.6-18.3-7.4-17.3-15.3-34.6-23.6-51.5C720 374.3 779.6 298 787.1 287.8zM512 157.2c90.3 0 172.8 33.9 235.5 89.5-6.4 9.1-59.9 81-186.2 128.4-58.2-107-122.7-194.8-132.6-208 27.3-6.6 55.2-9.9 83.3-9.9zM360.9 191c9.4 12.8 72.9 100.9 131.7 205.5C326.4 440.6 180 440 164.1 439.8c23.1-110.3 97.4-201.9 196.8-248.8zM156.7 512.5c0-3.6.1-7.3.2-10.9 15.5.3 187.7 2.5 365.2-50.6 10.2 19.9 19.9 40.1 28.8 60.3-4.7 1.3-9.4 2.7-14 4.2C353.6 574.9 256.1 736.4 248 750.1c-56.7-63-91.3-146.3-91.3-237.6zM512 867.8c-82.2 0-157.9-28-218.1-75 6.4-13.1 78.3-152 278.7-221.9l2.3-.8c49.9 129.6 70.5 238.3 75.8 269.5A350.46 350.46 0 0 1 512 867.8zm198.5-60.7c-3.6-21.6-22.5-125.6-69-253.3C752.9 536 850.7 565.2 862.8 569c-15.8 98.8-72.5 184.2-152.3 238.1z"/>
            </svg>
        }
    }
}
