// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct TableLink {}

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

impl Component for TableLink {
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
                data-icon={ "TableLink" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 1024V896H640v768H0V128h1664v896h-128V896h-384v128h-128zm512-256V512h-384v256h384zM512 1536v-256H128v256h384zm0-384V896H128v256h384zm0-384V512H128v256h384zM128 384h1408V256H128v128zm512 384h384V512H640v256zm1408 960q0 66-25 124t-68 102-102 69-125 25h-256q-67 0-125-25t-101-68-69-102-25-125q0-57 19-109t53-93 81-71 103-41v133q-58 21-93 69t-35 112q0 40 15 75t41 61 61 41 75 15h256q40 0 75-15t61-41 41-61 15-75q0-65-37-113t-97-70q6-36 6-73 0-15-1-29t-3-29q57 12 104 40t82 70 54 93 20 111zM896 1472q0 65 37 113t97 70q-6 36-6 73 0 15 1 29t3 29q-56-9-104-38t-82-71-54-96-20-109q0-66 25-124t68-102 102-69 125-25h256q66 0 124 25t101 69 69 102 26 124q0 54-20 105t-56 94-81 72-99 43v-133q42-9 67-23t38-36 18-52 5-70q0-40-15-75t-41-61-61-41-75-15h-256q-40 0-75 15t-61 41-41 61-15 75z" />
            </svg>
        }
    }
}
