// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Trophy2 {}

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

impl Component for Trophy2 {
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
                data-icon={ "Trophy2" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1792 256q27 0 50 10t40 27 28 41 10 50v192q0 86-30 163t-85 137-127 98-159 48q-19 62-51 116t-76 98-97 77-114 56q35 34 58 76t35 91h70q40 0 75 15t61 41 41 61 15 75v192H384v-192q0-40 15-75t41-61 61-41 75-15h70q11-48 34-90t59-77q-61-22-114-55t-96-78-76-98-52-116q-86-9-159-47t-127-99-84-137T0 576V384q0-27 10-50t27-40 41-28 50-10h256V128h1152v128h256zm-1280 0v128h896V256H512zM128 576q0 57 19 109t53 93 81 71 103 41V384H128v192zm1280 1152q0-26-19-45t-45-19H576q-26 0-45 19t-19 45v64h896v-64zm-267-192q-10-29-28-52t-42-41-52-26-59-9q-30 0-58 9t-53 26-42 40-28 53h362zm-181-256q81 0 161-27t144-76 103-121 40-160V512H512v384q0 89 39 160t103 120 144 77 162 27zm832-896h-256v506q56-12 103-41t81-70 53-94 19-109V384z" />
            </svg>
        }
    }
}
