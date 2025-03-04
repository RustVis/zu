// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct TestExploreSolid {}

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

impl Component for TestExploreSolid {
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
                data-icon={ "TestExploreSolid" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1472 896q119 0 224 45t183 124 123 183 46 224q0 119-45 224t-124 183-183 123-224 46q-119 0-224-45t-183-124-123-183-46-224q0-119 45-224t124-183 183-123 224-46zm64 1052q81-12 151-47t124-90 89-124 48-151h-156v-128h156q-12-81-47-151t-90-124-124-89-151-48v156h-128V996q-81 12-151 47t-124 90-89 124-48 151h156v128H996q12 81 47 151t90 124 124 89 151 48v-156h128v156zm-192-476q0-38 20-70t55-47l309-139-139 309q-15 34-47 54t-70 21q-27 0-50-10t-40-27-28-41-10-50zm-576 16q0 82 18 160t52 150 84 135 113 115H354q-40 0-75-15t-61-41-42-61-15-75q0-27 7-51t21-48l569-990q10-18 10-35V128H640V0h768v128h-128v604q0 17 8 31t17 29q-32 9-62 20t-60 25q-15-20-23-50t-8-55V128H896v604q0 52-28 100l-330 576h235q-2 20-3 40t-2 40z" />
            </svg>
        }
    }
}
