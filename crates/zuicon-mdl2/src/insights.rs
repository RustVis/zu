// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Insights {}

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

impl Component for Insights {
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
                data-icon={ "Insights" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 384q119 0 224 45t183 124 123 183 46 224q0 63-8 118t-25 105-44 99-64 100q-29 40-51 72t-36 64-21 70-7 89v179q0 40-15 75t-41 61-61 41-75 15H832q-40 0-75-15t-61-41-41-61-15-75v-180q0-51-7-88t-21-69-36-65-51-72q-37-51-63-99t-44-99-26-106-8-118q0-119 45-224t124-183 183-123 224-46zm192 1472v-64H768v64q0 26 19 45t45 19h256q26 0 45-19t19-45zm256-896q0-93-35-174t-96-143-142-96-175-35q-93 0-174 35t-143 96-96 142-35 175q0 89 18 153t47 114 61 94 61 92 48 108 21 143h384q1-83 20-142t48-108 61-92 61-94 47-115 19-153zM960 256q-26 0-45-19t-19-45V64q0-26 19-45t45-19q26 0 45 19t19 45v128q0 26-19 45t-45 19zM192 928H64q-26 0-45-19T0 864q0-26 19-45t45-19h128q26 0 45 19t19 45q0 26-19 45t-45 19zm53 261q26 0 45 19t19 46q0 20-11 35t-30 24q-11 5-30 13t-41 17-40 15-32 7q-26 0-45-19t-19-46q0-20 11-35t30-24q11-4 30-13t41-17 40-15 32-7zm152-645q0 26-19 45t-45 19q-18 0-33-9l-109-67q-14-9-22-23t-9-32q0-26 19-45t45-19q16 0 33 10l110 66q14 8 22 23t8 32zm83-368q0-26 19-45t45-19q17 0 32 9t24 24l62 112q8 14 8 30 0 27-19 46t-45 19q-17 0-32-9t-24-24l-62-112q-8-14-8-31zm1376 624q26 0 45 19t19 45q0 26-19 45t-45 19h-128q-26 0-45-19t-19-45q0-26 19-45t45-19h128zm2 501q0 26-19 45t-45 19q-11 0-30-6t-41-16-40-17-31-14q-18-8-29-24t-12-36q0-27 19-45t46-19q12 0 31 7t40 16 40 18 31 13q18 8 29 23t11 36zm-271-693q-26 0-45-19t-19-45q0-17 8-32t22-23l110-66q17-10 33-10 26 0 45 19t19 45q0 17-8 31t-23 24l-109 67q-15 9-33 9zm-337-321q0-16 8-30l62-112q8-15 23-24t33-9q26 0 45 19t19 45q0 17-8 31l-62 112q-8 15-23 24t-33 9q-26 0-45-19t-19-46z" />
            </svg>
        }
    }
}
