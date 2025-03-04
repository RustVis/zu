// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BranchSearch {}

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

impl Component for BranchSearch {
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
                data-icon={ "BranchSearch" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1274 1018q-11 57-39 105t-71 83-94 54-110 20H576q-32 0-61 10t-53 28-42 43-27 56q54 13 99 42t78 71 51 92 19 106q0 66-25 124t-69 102-102 69-124 25q-66 0-124-25t-101-68-69-102-25-125q0-57 19-108t52-94 81-71 103-40V633q-56-11-103-40t-80-71-53-93T1 320q0-66 25-124T94 95t102-69T320 0q66 0 124 25t101 69 69 102 26 124q0 57-19 109t-53 93-81 71-103 40v585q42-32 91-49t101-17h384q32 0 61-10t53-28 42-43 27-56q-54-13-99-42t-78-70-51-92-19-107q0-66 25-124t68-101 102-69 125-26q66 0 124 25t101 69 69 102 26 124q0 58-19 110t-55 94-83 71-105 39zm-250-314q0 40 15 75t41 61 61 41 75 15q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75zM128 320q0 40 15 75t41 61 61 41 75 15q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75zm384 1408q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75q0 40 15 75t41 61 61 41 75 15q40 0 75-15t61-41 41-61 15-75zm1216-704q66 0 124 25t101 69 69 102 26 124q0 66-25 124t-69 102-102 69-124 25q-47 0-92-13t-84-40l-419 418q-19 19-45 19t-45-19-19-45q0-26 19-45l418-419q-26-39-39-84t-14-92q0-66 25-124t68-101 102-69 125-26zm0 512q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75q0 40 15 75t41 61 61 41 75 15z" />
            </svg>
        }
    }
}
