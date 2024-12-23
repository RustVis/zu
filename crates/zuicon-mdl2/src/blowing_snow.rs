// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BlowingSnow {}

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

impl Component for BlowingSnow {
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
                data-icon={ "BlowingSnow" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1536 640q0 80-30 149t-82 122-123 83-149 30H0V896h1152q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100H768q0-79 30-149t82-122 122-83 150-30q79 0 149 30t122 82 83 123 30 149zm256 256q53 0 99 20t82 55 55 81 20 100q0 53-20 99t-55 82-81 55-100 20h-162q34 58 34 128 0 53-20 99t-55 82-81 55-100 20q-53 0-99-20t-82-55-55-81-20-100h128q0 26 10 49t27 41 41 28 50 10q26 0 49-10t41-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10H0v-128h1792q26 0 49-10t41-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50h-128q0-53 20-99t55-82 81-55 100-20zm-890 937q32 19 32 55 0 27-19 45t-45 19q-12 0-26-5t-27-14-26-16-23-14q0 19 1 44t-3 47-18 38-44 16q-30 0-44-15t-18-38-4-48 2-44q-10 6-22 14t-26 16-28 13-26 6q-26 0-45-18t-19-46q0-36 32-55l70-41-70-41q-32-19-32-55 0-27 18-45t46-19q13 0 26 5t27 13 26 16 23 15q0-19-1-44t2-47 19-38 44-16q30 0 43 15t18 38 4 48-1 44q10-6 22-14t26-16 27-13 27-6q27 0 45 18t19 46q0 36-32 55l-70 41 70 41zM154 544q-26 0-45-18t-19-46q0-36 32-55l70-41-70-41q-32-19-32-55 0-27 18-45t46-19q13 0 26 5t27 13 26 16 23 15q0-19-1-44t2-47 19-38 44-16q30 0 43 15t18 38 4 48-1 44q10-6 22-14t26-16 27-13 27-6q26 0 45 18t20 46q0 18-9 32t-24 23l-70 41 70 41q15 9 24 23t9 32q0 26-19 45t-46 19q-12 0-26-5t-27-14-26-16-23-14q0 19 1 44t-3 47-18 38-44 16q-30 0-44-15t-18-38-4-48 2-44q-10 6-22 14t-26 16-28 13-26 6z" />
            </svg>
        }
    }
}
