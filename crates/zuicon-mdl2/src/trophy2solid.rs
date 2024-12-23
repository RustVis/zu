// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Trophy2solid {}

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

impl Component for Trophy2solid {
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
                data-icon={ "Trophy2Solid" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M640 1536q0-62 29-109t76-80 103-50 112-17q55 0 111 17t103 49 76 80 30 110H640zm864 128q13 0 22 9t10 23v224H384v-224q0-13 9-22t23-10h1088zm288-1408q27 0 50 10t40 27 28 41 10 50v192q0 98-36 180t-98 141-147 93-180 34q-41 62-96 110t-121 80-137 49-145 17q-74 0-145-16t-137-49-120-81-97-110q-96 0-180-33t-146-93-99-142T0 576V384q0-27 10-50t27-40 41-28 50-10h256V128h1152v128h256zM384 384H128v192q0 57 19 109t53 93 81 71 103 41V384zm1408 0h-256v506q56-12 103-41t81-70 53-94 19-109V384z" />
            </svg>
        }
    }
}
