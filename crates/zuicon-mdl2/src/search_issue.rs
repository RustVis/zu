// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SearchIssue {}

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

impl Component for SearchIssue {
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
                data-icon={ "SearchIssue" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1664 576q0 119-45 224t-124 183-183 123-224 46q-99 0-191-32t-169-94l-619 619q-19 19-45 19t-45-19-19-45q0-26 19-45l619-619q-62-77-94-169t-32-191q0-119 45-224t124-183T864 46t224-46q119 0 224 45t183 124 123 183 46 224zm-1024 0q0 93 35 174t96 142 142 96 175 36q93 0 174-35t142-96 96-142 36-175q0-93-35-174t-96-142-142-96-175-36q-93 0-174 35t-142 96-96 142-36 175zm1024 1344h128v128h-128v-128zm64-704q53 0 99 20t82 55 55 81 20 100q0 47-17 89t-49 76h-1l-95 96q-14 14-20 28t-8 29-3 32 1 34h-128v-48q0-46 14-81t35-63 47-50 46-45 36-45 14-52q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50h-128q0-53 20-99t55-82 81-55 100-20z" />
            </svg>
        }
    }
}
