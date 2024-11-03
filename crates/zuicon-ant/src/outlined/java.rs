// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Java {}

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

impl Component for Java {
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
                data-icon={ "java" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M212.68 692.99s-34.325 19.95 24.343 26.6c71.1 8.05 107.351 7 185.632-7.874 0 0 20.665 12.949 49.385 24.149-175.475 75.074-397.184-4.375-259.36-42.875m-21.366-98.173s-38.352 28.35 20.315 34.475c75.83 7.875 135.897 8.4 239.571-11.55 0 0 14.36 14.525 36.952 22.4-212.427 62.123-448.846 5.075-296.838-45.325m180.73-166.422c43.256 49.699-11.384 94.498-11.384 94.498s109.804-56.7 59.368-127.573c-47.11-66.15-83.185-99.049 112.255-212.273.175 0-306.819 76.65-160.24 245.348M604.26 765.439s25.393 20.825-27.846 37.1c-101.397 30.625-421.7 39.9-510.664 1.225-32.048-13.825 28.02-33.25 46.934-37.275 19.613-4.2 30.997-3.5 30.997-3.5-35.551-25.025-229.94 49.175-98.771 70.35 357.605 58.1 652.165-26.075 559.35-67.9M229.142 493.144S66.1 531.818 171.35 545.818c44.482 5.95 133.095 4.55 215.58-2.275 67.423-5.6 135.196-17.85 135.196-17.85s-23.818 10.15-40.98 21.875C315.653 591.143-3.95 570.843 87.99 526.393c77.93-37.45 141.151-33.25 141.151-33.25M521.6 656.416c168.296-87.324 90.365-171.322 36.077-159.948-13.31 2.8-19.264 5.25-19.264 5.25s4.903-7.7 14.36-11.025c107.351-37.8 190.01 111.299-34.675 170.273 0-.175 2.627-2.45 3.502-4.55M420.028 0s93.166 93.1-88.438 236.246c-145.53 114.8-33.274 180.424 0 255.148-84.936-76.65-147.28-144.024-105.425-206.848C287.634 192.672 457.68 147.873 420.028 0m-174.25 893.188c161.466 10.325 409.443-5.775 415.222-82.075 0 0-11.208 28.875-133.445 51.975-137.824 25.9-307.87 22.925-408.567 6.3 0-.175 20.665 16.975 126.79 23.8" transform="translate(182 64)"/>
            </svg>
        }
    }
}
