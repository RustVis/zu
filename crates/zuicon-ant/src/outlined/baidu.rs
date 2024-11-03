// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Baidu {}

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

impl Component for Baidu {
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
                data-icon={ "baidu" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M118.02 448.043c92.37-19.802 79.767-130.078 76.948-154.187-4.56-37.201-48.258-102.157-107.628-97.02C12.631 203.547 1.686 311.42 1.686 311.42-8.347 361.297 25.898 467.845 118.02 448.043m84.7 214.138c10.282 38.692 43.947 40.432 43.947 40.432h48.341V584.549h-51.74c-23.218 6.96-34.495 25.104-36.982 32.81-2.737 7.787-8.707 27.589-3.566 44.822m169.07-531.11C371.79 58.66 330.663 0 279.585 0c-50.911 0-92.205 58.66-92.205 131.072 0 72.495 41.294 131.154 92.205 131.154 51.078 0 92.205-58.659 92.205-131.154m248.092 9.107c8.872-54.93-35.075-118.892-83.333-129.828C488.208-.753 427.843 76.63 422.37 127.088c-6.551 61.725 8.789 123.283 76.865 132.066 68.158 8.865 112.022-63.879 120.646-118.976m46.35 433.02S560.762 491.672 499.237 403.6c-83.416-129.911-201.989-77.052-241.624-11.019-39.469 66.033-100.994 107.873-109.7 118.892-8.873 10.937-127.363 74.816-101.078 191.554 26.285 116.656 118.739 114.501 118.739 114.501s68.076 6.711 147.097-10.936c79.02-17.565 147.014 4.391 147.014 4.391s184.576 61.725 235.073-57.168c50.414-118.975-28.524-180.617-28.524-180.617M230.416 750.17c-51.824-10.357-72.47-45.652-75.124-51.7-2.57-6.13-17.247-34.55-9.453-82.852 22.388-72.412 86.235-77.632 86.235-77.632h63.847v-78.46l54.395.828.083 289.816zm205.388-.829c-53.565-13.753-56.052-51.782-56.052-51.782V544.946l56.052-.912v137.12c3.4 14.582 21.642 17.316 21.642 17.316h56.882V544.946h59.618V749.34zM759.64 351.617c0-26.347-21.89-105.72-103.15-105.72-81.426 0-92.289 74.899-92.289 127.841 0 50.54 4.312 121.13 105.39 118.81 101.16-2.155 90.049-114.419 90.049-140.931" transform="translate(132 99)"/>
            </svg>
        }
    }
}
