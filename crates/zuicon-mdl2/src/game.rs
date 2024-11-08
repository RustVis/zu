// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Game {}

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

impl Component for Game {
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
                data-icon={ "Game" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1879 553q82 82 125 186t44 221q0 35-2 84t-8 106-19 117-33 119q-30 86-70 140t-83 85-87 42-82 11q-33 0-60-11t-52-30-46-41-43-48q-21-24-41-46t-43-39-46-29-54-12H769q-29 1-53 12t-47 28-42 40-42 46q-21 24-42 47t-46 42-52 29-61 12q-38 0-82-11t-87-41-83-85-70-141q-20-58-32-118t-19-118-9-106-2-84q0-116 43-220t126-187q81-82 186-125t221-44h896q116 0 220 43t187 126zm-14 791q19-53 30-110t16-109 7-96 2-69q0-93-35-174t-96-142-142-96-175-36H576q-93 0-174 35t-142 96-96 142-36 175q0 26 1 69t7 95 17 110 30 110q25 72 55 111t58 57 52 21 36 3q22 0 48-26t56-60q24-28 51-57t61-54 75-41 91-18h516q50 2 91 18t74 40 61 54 52 58q29 33 55 59t49 27q12 0 35-3t52-21 59-57 55-111zM512 896h128v128H512v128H384v-128H256V896h128V768h128v128zm1152 128q27 0 50 10t40 27 28 41 10 50q0 27-10 50t-27 40-41 28-50 10q-27 0-50-10t-40-27-28-41-10-50q0-27 10-50t27-40 41-28 50-10zm-256-256q27 0 50 10t40 27 28 41 10 50q0 27-10 50t-27 40-41 28-50 10q-27 0-50-10t-40-27-28-41-10-50q0-27 10-50t27-40 41-28 50-10z" />
            </svg>
        }
    }
}
