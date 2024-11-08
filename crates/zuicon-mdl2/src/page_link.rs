// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PageLink {}

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

impl Component for PageLink {
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
                data-icon={ "PageLink" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M901 1472q0 65 37 113t97 70q-3 18-5 36t-2 37q0 14 1 29t4 29q-57-11-104-39t-82-70-54-94-19-111q0-66 25-124t67-101 101-69 124-26h254q65 0 123 25t101 70 68 102 25 123q0 56-19 108t-52 94-81 71-102 40v-133q57-22 92-69t35-111q0-39-15-74t-40-61-60-42-75-15h-254q-40 0-75 15t-60 41-40 61-15 75zm1147 253q0 66-25 125t-68 103-102 69-125 26h-256q-67 0-125-25t-101-70-69-103-25-125q0-56 19-108t53-95 81-73 103-40v133q-29 10-52 28t-40 43-26 53-10 59q0 40 15 75t41 62 61 42 75 16h256q40 0 75-15t61-43 41-62 15-75q0-31-10-60t-27-54-43-43-55-28q3-18 5-36t2-37q0-15-2-29t-4-29q57 11 105 40t83 71 54 94 20 111zM128 128v1792h896v128H0V0h1115l549 549v475h-128V640h-512V128H128zm1024 91v293h293l-293-293z" />
            </svg>
        }
    }
}
