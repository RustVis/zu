// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Fingerprint {}

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

impl Component for Fingerprint {
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
                data-icon={ "Fingerprint" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 1216q0 119-45 224t-124 183-183 123-224 46q-26 0-45-19t-19-45q0-26 19-45t45-19q93 0 174-35t142-96 96-142 36-175q0-26 19-45t45-19q26 0 45 19t19 45zm-64-960q-131 0-248 31t-234 89q-16 8-30 8-26 0-45-19t-19-45q0-37 34-56 58-34 125-59t137-43 142-25 138-9q67 0 138 8t141 26 137 42 126 60q34 19 34 55 0 27-18 46t-46 19q-10 0-16-1t-14-7q-115-59-233-89t-249-31zm0 128q112 0 219 29t203 86q43 26 86 59t83 72 72 82 58 88q8 16 8 31 0 27-19 46t-45 19q-19 0-32-8t-23-24q-20-31-40-60t-45-56q-50-56-110-100t-127-74-140-46-148-16q-121 0-234 40T519 668q-23 18-43 38t-41 41q-11 11-21 16t-26 5q-27 0-45-19t-19-45q0-13 4-23t12-20q24-31 57-60t69-55 75-48 74-39q81-38 168-56t177-19zm0 256q119 0 224 45t183 124 123 183 46 224q0 94-15 186t-48 181q-13 36-32 79t-43 85-49 82-51 67q-11 11-22 17t-28 7q-26 0-45-18t-19-46q0-21 14-40 19-26 37-51t35-53q69-113 103-238t35-258q0-93-35-174t-96-142-142-96-175-36q-93 0-174 35t-142 96-96 142-36 175q0 26-19 45t-45 19q-26 0-45-19t-19-45q0-119 45-224t124-183 183-123 224-46zm0 256q66 0 124 25t101 69 69 102 26 124q0 103-25 203t-74 190-117 167-156 133q-8 5-16 8t-19 3q-27 0-45-19t-19-45q0-18 7-30t21-23q27-22 53-41t52-45q101-100 155-229t55-272q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75q0 66-25 124t-68 102-102 69-125 25q-26 0-45-19t-19-45q0-26 19-45t45-19q40 0 75-15t61-41 41-61 15-75q0-66 25-124t68-101 102-69 125-26z" />
            </svg>
        }
    }
}
