// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Discord {}

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

impl Component for Discord {
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
                data-icon={ "discord" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M693.152 0c51.153 0 92.41 41.358 94.848 90.028V873l-97.395-82.68-53.482-48.67-58.356-50.852 24.376 80.207H92.41C41.403 771.005 0 732.265 0 680.94V90.21C0 41.54 41.476.11 92.592.11h600.305zM470.164 207.109h-1.091l-7.346 7.269c75.382 21.806 111.855 55.86 111.855 55.86-48.582-24.277-92.364-36.416-136.146-41.323-31.636-4.906-63.272-2.326-90 0h-7.272c-17.091 0-53.455 7.27-102.182 26.713-16.982 7.378-26.727 12.212-26.727 12.212s36.436-36.417 116.727-55.86l-4.91-4.907s-60.8-2.326-126.436 46.156c0 0-65.636 114.265-65.636 255.134 0 0 36.364 63.238 136.11 65.637 0 0 14.545-19.371 29.272-36.417-56-17.008-77.818-51.026-77.818-51.026s4.872 2.398 12.181 7.269h2.182c1.091 0 1.6.545 2.182 1.09v.218c.582.581 1.091 1.09 2.182 1.09 12 4.943 24 9.813 33.818 14.538a297.576 297.576 0 0 0 65.455 19.48c33.818 4.906 72.581 7.269 116.727 0 21.818-4.906 43.636-9.704 65.454-19.444 14.182-7.269 31.637-14.537 50.8-26.785 0 0-21.818 34.018-80.181 51.026 12 16.937 28.909 36.344 28.909 36.344 99.782-2.18 138.545-65.419 140.727-62.73 0-140.65-66-255.133-66-255.133-59.455-44.121-115.09-45.793-124.91-45.793l2.037-.727zM477 367c25.463 0 46 21.757 46 48.41 0 26.833-20.646 48.59-46 48.59s-46-21.757-46-48.373c.072-26.834 20.754-48.518 46-48.518zm-165.855 0C336.499 367 357 388.757 357 415.41c0 26.833-20.646 48.59-46 48.59s-46-21.757-46-48.373c0-26.834 20.646-48.518 46-48.518z" transform="translate(118 87)"/>
            </svg>
        }
    }
}