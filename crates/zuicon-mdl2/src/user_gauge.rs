// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct UserGauge {}

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

impl Component for UserGauge {
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
                data-icon={ "UserGauge" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M768 1024q-88 0-170 23t-153 64-129 100-100 130-65 153-23 170H0q0-119 35-230t101-206 156-167 204-115q-115-75-177-186t-63-248q0-106 40-199t109-163T568 40 768 0q106 0 199 40t163 109 110 163 40 200q0 66-16 129t-48 119-75 103-101 83q55 21 105 51t97 67q-69 24-133 62-30-24-72-43t-90-32-95-20-84-7zm0-128q80 0 150-30t122-82 82-122 30-150q0-79-30-149t-82-122-123-83-149-30q-80 0-150 30t-122 82-82 122-30 150q0 80 30 150t82 122 122 82 150 30zm1163 464l-286 286q19 41 19 82 0 40-15 75t-41 61-61 41-75 15q-40 0-75-15t-61-41-41-61-15-75q0-40 15-75t41-61 61-41 75-15q41 0 82 19l286-286 91 91zm-459 432q26 0 45-19t19-45q0-26-19-45t-45-19q-26 0-45 19t-19 45q0 26 19 45t45 19zm0-512q-93 0-174 35t-142 96-96 142-36 175q0 64 18 125t53 116l-102 79q-48-72-72-153t-25-167q0-119 45-224t124-183 183-123 224-46q65 0 127 15t120 43l-98 97q-36-12-73-19t-76-8zm518 201q28 58 43 120t15 127q0 85-24 166t-73 154l-102-79q34-54 52-115t19-126q0-38-7-75t-20-74l97-98z" />
            </svg>
        }
    }
}
