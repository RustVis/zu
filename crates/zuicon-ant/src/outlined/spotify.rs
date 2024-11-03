// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Spotify {}

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

impl Component for Spotify {
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
                data-icon={ "spotify" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M448 0C200.517 0 0 200.517 0 448s200.517 448 448 448 448-200.517 448-448S695.483 0 448 0m0 74.664a371.855 371.855 0 0 1 264.427 108.91A371.855 371.855 0 0 1 821.337 448a371.855 371.855 0 0 1-108.91 264.427A371.855 371.855 0 0 1 448 821.337a371.855 371.855 0 0 1-264.427-108.91A371.855 371.855 0 0 1 74.663 448a371.855 371.855 0 0 1 108.91-264.427A371.855 371.855 0 0 1 448 74.663M388.49 252c-72.613 0-135.893 6.72-196 25.685-15.903 3.174-29.157 15.158-29.157 37.334 0 22.138 16.352 41.701 38.491 38.453 9.483 0 15.904-3.472 22.176-3.472 50.587-12.693 107.632-18.667 164.49-18.667 110.545 0 224 24.64 299.825 68.843 9.482 3.21 12.693 6.981 22.176 6.981 22.176 0 37.632-16.314 40.842-38.49 0-18.966-9.482-31.062-22.176-37.334C634.368 277.648 508.517 252 388.491 252M378 390.843c-66.341 0-113.605 9.482-161.019 22.176-15.717 6.234-24.49 16.053-24.49 34.981 0 15.755 12.544 31.51 31.509 31.51 6.421 0 9.184-.3 18.667-3.51 34.72-9.483 82.394-15.157 133.018-15.157 104.235 0 194.955 25.386 261.334 66.49 6.234 3.211 12.693 5.824 22.138 5.824 18.966 0 31.51-16.053 31.51-34.981 0-12.693-5.974-25.237-18.667-31.51-82.133-50.586-186.517-75.823-294-75.823m10.49 136.49c-53.647 0-104.533 5.974-155.157 18.667-12.693 3.21-22.176 12.245-22.176 28 0 12.693 9.931 25.685 25.686 25.685 3.21 0 12.394-3.509 18.666-3.509a581.728 581.728 0 0 1 129.51-15.195c78.885 0 151.05 18.966 211.157 53.686 6.421 3.21 13.552 5.824 19.824 5.824 12.693 0 24.79-9.483 28-22.139 0-15.904-6.87-21.765-16.352-28-69.552-41.141-150.79-63.019-239.157-63.019" transform="translate(64 64)"/>
            </svg>
        }
    }
}