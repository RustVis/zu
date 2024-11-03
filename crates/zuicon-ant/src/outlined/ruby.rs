// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Ruby {}

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

impl Component for Ruby {
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
                data-icon={ "ruby" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M397.815.02c-.735.049-1.469.12-2.2.213h-4.328l-3.406 1.703a36.334 36.334 0 0 0-8.87 4.4l-145.96 73.013L79.356 233.041 6.696 378.288a36.333 36.333 0 0 0-4.897 9.863l-1.561 3.122v3.974a36.332 36.332 0 0 0 0 8.302v298.229l6.883 9.508c5.975 8.28 12.713 16.544 20.578 24.41 37.856 37.854 87.664 57.169 142.625 62.015a36.335 36.335 0 0 0 11.566 1.774h575.753c3.14.534 6.337.654 9.508.355a36.335 36.335 0 0 0 2.554-.355h29.803V769.54a36.332 36.332 0 0 0 0-11.92V181.88a36.332 36.332 0 0 0-1.774-11.566c-4.848-54.956-24.165-104.757-62.017-142.622h-.071v-.07c-7.85-7.807-16.071-14.49-24.268-20.436l-9.58-6.954H403.138a36.335 36.335 0 0 0-5.322-.213m133.188 72.872h145.96c2.467 2.081 5.248 4.054 7.451 6.245 26.585 26.63 40.964 64.743 42.291 111.188zm-132.691 5.11 65.707 39.38-25.474 156.104-64.359 64.357-100.69 100.687-156.107 25.473-39.381-65.705 61.095-122.258L276.05 139.095zm132.762 79.612 123.183 73.937-138.084 17.242zm178.814 140.21c-21.21 68.248-62.66 142.573-122.402 211.875l-65.85-188.389zm-252.54 59.603 53.645 153.55-153.553-53.643 68.12-68.119zm269.499 81.032v236.994L626.44 575.05c40.102-43.738 73.727-89.827 100.406-136.59m-478.044 77.697-17.242 138.081-73.938-123.18zm72.52 5.464 188.322 65.847c-69.283 59.712-143.574 101.195-211.81 122.4zm-248.424 9.366 117.435 195.698c-46.5-1.327-84.636-15.736-111.262-42.361-2.161-2.162-4.113-4.939-6.173-7.38zm502.169 95.436 100.405 100.404h-237c46.768-26.68 92.86-60.308 136.595-100.404" transform="translate(112 112)"/>
            </svg>
        }
    }
}
