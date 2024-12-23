// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct HeadsetSolid {}

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

impl Component for HeadsetSolid {
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
                data-icon={ "HeadsetSolid" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 0q106 0 204 27t183 78 156 120 120 155 77 184 28 204v544q0 40-28 68t-68 28h-288V768h256q0-88-23-170t-64-153-100-129-130-100-153-65-170-23q-88 0-170 23t-153 64-129 100-100 130-65 153-23 170h256v640H512v128q0 53 20 99t55 82 81 55 100 20v-48q0-33 23-56t57-24h352q33 0 56 23t24 57v224q0 33-23 56t-57 24H848q-33 0-56-23t-24-57v-48q-80 0-150-30t-122-82-82-122-30-150v-128h-32q-40 0-68-28t-28-68V768q0-106 27-204t78-183 120-156 155-120 184-77 204-28z" />
            </svg>
        }
    }
}
