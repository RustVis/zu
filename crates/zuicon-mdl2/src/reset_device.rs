// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ResetDevice {}

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

impl Component for ResetDevice {
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
                data-icon={ "ResetDevice" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1280 1792v-128h256v128h-256zm-512-640H640V768h128v384zm-128 128h128v128H640v-128zm640-192q0 119-45 224t-124 183-183 123-224 46q-119 0-224-45t-183-124-123-183-46-224v-64h128v64q0 93 35 174t96 142 142 96 175 36q93 0 174-35t142-96 96-142 36-175q0-93-35-174t-96-142-142-96-175-36H282l147 147-90 90L37 576l302-301 90 90-147 147h422q119 0 224 45t183 124 123 183 46 224zM1920 0q27 0 50 10t40 27 28 41 10 50v1792q0 27-10 50t-27 40-41 28-50 10H896q-27 0-50-10t-40-27-28-41-10-50v-99q32-3 64-9t64-14v122h1024V128H896v250q-32-8-64-14t-64-9V128q0-27 10-50t27-40 41-28 50-10h1024z" />
            </svg>
        }
    }
}
