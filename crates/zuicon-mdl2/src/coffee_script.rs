// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CoffeeScript {}

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

impl Component for CoffeeScript {
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
                data-icon={ "CoffeeScript" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M576 544q-26 0-45-19t-19-45q0-23-14-41t-33-30q-35-24-71-48t-68-52-50-65-20-84q0-26 19-45t45-19q26 0 45 19t19 45q0 23 14 41t33 30q34 24 71 48t68 52 50 65 20 84q0 26-19 45t-45 19zm448 0q-26 0-45-19t-19-45q0-23-14-41t-33-30q-35-24-71-48t-68-52-50-65-20-84q0-26 19-45t45-19q26 0 45 19t19 45q0 23 14 41t33 30q34 24 71 48t68 52 50 65 20 84q0 26-19 45t-45 19zm448 0q-26 0-45-19t-19-45q0-23-14-41t-33-30q-35-24-71-48t-68-52-50-65-20-84q0-26 19-45t45-19q26 0 45 19t19 45q0 23 14 41t33 30q34 24 71 48t68 52 50 65 20 84q0 26-19 45t-45 19zm133 736q-33 75-82 138t-110 118h443v128q0 45-14 90t-40 82-65 60-89 24H208q-50 0-88-23t-65-60-41-82-14-91v-128h379q-118-103-184-239t-67-294V640h1605q66 0 123 25t100 69 67 102 25 124q0 65-24 123t-67 102-100 69-124 26h-128zm59-512q0 42 1 84t1 85q0 55-3 109t-16 106h86q39 0 73-15t60-42 39-61 15-74q0-39-14-74t-40-61-59-41-74-16h-69zM256 1003q0 88 26 169t75 150 114 123 145 91h560q79-36 145-90t114-124 74-150 27-169V768H256v235zm1392 789q21 0 36-14t25-33 14-42 5-39H128q0 17 4 39t15 41 25 34 36 14h1440z" />
            </svg>
        }
    }
}
