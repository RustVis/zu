// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct InternetSharing {}

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

impl Component for InternetSharing {
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
                data-icon={ "InternetSharing" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M896 640h128v1408H896V640zM372 244q-58 58-103 126t-77 144-47 155-17 163q0 82 16 162t48 156 76 143 104 127l-91 91q-136-137-208-311T0 832q0-193 72-367t209-312l91 91zm1267-91q136 137 208 311t73 368q0 193-72 367t-209 312l-91-91q58-58 103-126t77-144 47-155 17-163q0-82-16-162t-48-156-76-143-104-127l91-91zm-272 272q81 81 125 186t44 221q0 115-44 220t-125 187l-90-90q63-63 96-145t34-172q0-89-33-171t-97-146l90-90zm-724 90q-63 63-97 145t-34 172q0 89 34 171t97 146l-90 90q-81-81-125-186t-44-221q0-115 44-220t125-187l90 90z" />
            </svg>
        }
    }
}
