// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ParkingLocationMirrored {}

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

impl Component for ParkingLocationMirrored {
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
                data-icon={ "ParkingLocationMirrored" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M29 1459q-29 64-29 133v72q0 76 41 139t110 94q31 69 94 110t139 41q69 0 128-34t94-94h708q35 60 94 94t128 34q69 0 128-34t94-94h162q27 0 50-10t40-27 28-41 10-50v-256q0-80-30-150t-82-122-122-82-150-30h-37l-328-328q-27-27-62-41t-74-15H256v128h29L29 1459zm739-563v256H309l99-219q8-17 24-27t35-10h301zm395 0q26 0 45 19l237 237H896V896h267zm373 1024q-27 0-50-10t-40-27-28-41-10-50q0-27 10-50t27-40 41-28 50-10q27 0 50 10t40 27 28 41 10 50q0 27-10 50t-27 40-41 28-50 10zm0-384q-53 0-99 20t-82 55-55 81-20 100H896v-512h768q53 0 99 20t82 55 55 81 20 100v256h-128q0-53-20-99t-55-82-81-55-100-20zM384 1920q-27 0-50-10t-40-27-28-41-10-50q0-27 10-50t27-40 41-28 50-10q27 0 50 10t40 27 28 41 10 50q0 27-10 50t-27 40-41 28-50 10zm-256-328q0-41 17-80l106-232h517v512H640q0-53-20-99t-55-82-81-55-100-20q-42 0-81 13t-71 37-56 57-37 74q-11-27-11-53v-72z" />
            </svg>
        }
    }
}
