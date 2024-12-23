// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AndroidLogo {}

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

impl Component for AndroidLogo {
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
                data-icon={ "AndroidLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1229 186q64 33 117 80t91 106 60 127 21 141H384q0-73 21-141t59-126 92-106 118-81q-4-7-18-33t-31-55-28-53-13-28q0-17 16-17 11 0 16 9l89 162q118-52 246-52t247 52l89-162q4-9 16-9 16 0 16 17 0 4-12 28t-29 53-30 55-19 33zM704 384q26 0 45-19t19-45q0-26-19-45t-45-19q-26 0-45 19t-19 45q0 26 19 45t45 19zm512 0q26 0 45-19t19-45q0-26-19-45t-45-19q-26 0-45 19t-19 45q0 26 19 45t45 19zM384 704h1149v759q0 28-11 53t-29 43-44 30-53 11h-116v255q0 27-10 50t-27 41-41 28-50 10q-27 0-50-10t-40-28-28-41-10-50v-255H896v255q0 27-10 50t-27 41-41 28-51 10q-27 0-50-10t-40-28-27-41-10-50v-255H521q-29 0-53-10t-44-30-29-43-11-54V704zm-192 0q26 0 49 10t41 27 28 41 10 50v512q0 26-10 49t-27 41-41 28-50 10q-26 0-49-10t-41-27-28-41-10-50V832q0-26 10-49t27-41 41-28 50-10zm1536 0q26 0 49 10t41 27 28 41 10 50v512q0 26-10 49t-27 41-41 28-50 10q-26 0-49-10t-41-27-28-41-10-50V832q0-26 10-49t27-41 41-28 50-10z" />
            </svg>
        }
    }
}
