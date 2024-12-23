// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct RainSnow {}

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

impl Component for RainSnow {
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
                data-icon={ "RainSnow" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1511 1504q26 0 44 19t19 45q0 36-32 55l-70 41 70 41q32 19 32 55 0 27-19 45t-45 19q-13 0-26-5t-27-13-26-16-23-15q0 19 1 44t-3 47-18 38-44 16q-30 0-44-15t-18-38-4-48 2-44q-11 6-23 14t-25 16-27 13-27 6q-26 0-45-18t-19-46q0-36 32-55l70-41-70-41q-32-19-32-55 0-26 18-45t45-19q13 0 27 5t27 14 26 16 23 14q0-19-1-44t2-47 19-38 44-16q30 0 43 15t18 38 4 48-1 44q10-6 22-14t26-16 28-13 27-6zm-533 323q14 30 14 61 0 33-12 62t-35 51-51 34-62 13q-33 0-62-12t-51-35-34-51-13-62q0-31 14-61l146-291 146 291zm783-791q63 17 115 52t91 85 60 109 21 126q0 61-18 117t-52 104-81 83-105 56v-139q59-35 93-93t35-128q0-55-20-101t-57-81-83-54-102-20q-12-82-51-152t-98-122-134-81-159-29q-77 0-146 25t-127 69-98 106-61 135q-44-38-97-58t-111-21q-66 0-124 25t-102 68-69 102-25 125q0 66 25 124t68 102 102 69 125 25l-61 122q-83-11-154-50t-122-98-81-135-30-159q0-93 35-174t96-142 142-96 175-36q74 0 147 25 39-65 92-117t117-88 136-56 148-20q93 0 178 29t158 81 126 125 83 161z" />
            </svg>
        }
    }
}
