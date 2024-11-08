// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Cycling {}

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

impl Component for Cycling {
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
                data-icon={ "Cycling" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M384 1152q79 0 149 30t122 82 83 123 30 149q0 80-30 149t-82 122-123 83-149 30q-80 0-149-30t-122-82-83-122-30-150q0-79 30-149t82-122 122-83 150-30zm0 640q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 53 20 99t55 82 81 55 100 20zm1280-640q79 0 149 30t122 82 83 123 30 149q0 80-30 149t-82 122-123 83-149 30q-80 0-149-30t-122-82-83-122-30-150q0-79 30-149t82-122 122-83 150-30zm0 640q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 53 20 99t55 82 81 55 100 20zm-256-704q-26 0-45-19l-115-114-102 101q9 9 21 20t23 23 18 25 8 28v448q0 40-15 75t-41 61-61 41-75 15q-40 0-75-15t-61-41-41-61-15-75v-294l-54-54-55-55q-39-38-61-76t-22-96q0-30 6-54t17-45 27-40 37-39l429-429q-2-8-3-17t-1-17q0-53 20-99t55-82 81-55 100-20q53 0 99 20t82 55 55 81 20 100q0 42-13 80t-36 71-57 55-73 37l77 77h166q40 0 75 15t61 41 41 61 15 75q0 40-15 75t-41 61-61 41-75 15h-320zm0-832q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10zm-416 800q0-26 19-45l192-192q19-19 45-19t45 19l141 141h294q26 0 45-19t19-45q0-26-19-45t-45-19h-192q-26 0-45-19l-275-275-403 403q-18 18-31 36t-14 47q0 26 12 44t29 35l132 132q19 19 19 45v320q0 26 19 45t45 19q26 0 45-19t19-45v-422l-77-77q-19-19-19-45z" />
            </svg>
        }
    }
}
