// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ReleaseDefinition {}

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

impl Component for ReleaseDefinition {
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
                data-icon={ "ReleaseDefinition" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 0v64q0 167-25 310t-80 273-137 248-201 238q-38 38-77 75t-81 72h-210q75-54 143-112t134-124q99-101 173-203t124-213 76-234 32-265q-142 5-265 31t-233 77-213 123-203 173q-100 97-182 204T668 962l356 355v248q-138 80-279 156l-418-418q76-141 156-279H256V722l572-191v1q127-136 252-236t264-166 295-98 345-32h64zM973 1447l-372-372q-29 51-57 102t-58 102v1l282 282h1q51-29 102-57t102-58zM558 896q29-46 58-91t62-89l-294 98v82h174zm978-128q0 53-20 99t-55 82-81 55-100 20q-53 0-99-20t-82-55-55-81-20-100q0-53 20-99t55-82 81-55 100-20q53 0 99 20t82 55 55 81 20 100zm-384 0q0 27 10 50t27 40 41 28 50 10q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50zm-896 768q53 0 99 20t82 55 55 81 20 100q0 53-20 99t-55 82-81 55-100 20H0v-256q0-53 20-99t55-82 81-55 100-20zm0 384q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50v128h128zm1152-128v-128h640v128h-640zm0-256v-128h640v128h-640zm-256 512v-128h128v128h-128zm0-512v-128h128v128h-128zm256 512v-128h640v128h-640zm-256-256v-128h128v128h-128z" />
            </svg>
        }
    }
}
