// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Pinterest {}

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

impl Component for Pinterest {
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
                data-icon={ "pinterest" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M448 0C200.974 0 0 200.974 0 448c0 192.526 122.082 357.042 292.882 420.282-4.919-43.857-4.147-115.682 3.97-150.457 7.602-32.666 49.11-208.167 49.11-208.167S333.43 484.563 333.43 447.5c0-58.244 33.744-101.7 75.767-101.7 35.742 0 52.977 26.834 52.977 58.984 0 35.951-22.857 89.652-34.696 139.422-9.873 41.702 20.907 75.703 62.012 75.703 74.431 0 131.644-78.49 131.644-191.77 0-100.266-72.03-170.38-174.906-170.38-119.144 0-189.081 89.377-189.081 181.752 0 35.983 13.852 74.575 31.167 95.579 3.414 4.155 3.914 7.78 2.9 12-3.174 13.223-10.228 41.669-11.63 47.499-1.82 7.683-6.073 9.278-13.998 5.589-52.314-24.354-85.012-100.813-85.012-162.246 0-132.093 95.965-253.428 276.718-253.428 145.286 0 258.18 103.504 258.18 241.88 0 144.334-91.02 260.482-217.316 260.482-42.441 0-82.323-22.05-95.965-48.096 0 0-21.003 79.956-26.11 99.558-8.813 33.9-46.539 104.13-65.477 136.03A446.164 446.164 0 0 0 448 896c247.035 0 448-200.965 448-448C896 200.974 695.035 0 448 0" transform="translate(64 64)"/>
            </svg>
        }
    }
}
