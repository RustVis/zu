// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct TransferCall {}

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

impl Component for TransferCall {
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
                data-icon={ "TransferCall" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1402 1307q39 0 76 15t65 43l191 191q28 28 43 65t15 76q0 39-15 76t-43 65l-96 96q-63 62-143 88t-166 26q-107 0-221-33t-228-94-223-142-207-179-180-207-143-223-93-228T0 720q0-77 13-131t40-98 66-85 91-92q28-28 65-43t76-15q39 0 76 15t65 43l191 191q28 28 43 65t15 76q0 39-13 69t-32 56-42 46-42 40-33 37-13 39q0 29 21 50l478 478q21 21 50 21 21 0 39-13t37-32 39-42 46-42 55-33 71-13zm-71 613q60 0 101-11t74-34 65-54 72-73q21-21 21-50 0-30-21-51l-191-191q-21-21-51-21-21 0-39 13t-37 32-39 42-45 42-56 33-70 13q-40 0-76-15t-65-44l-477-477q-28-28-43-64t-16-77q0-39 13-69t32-56 42-45 42-40 33-37 13-40q0-29-21-50L401 405q-21-21-50-21-30 0-51 21-41 41-73 72t-54 64-33 75-12 101q0 95 31 197t86 204 130 202 165 188 188 164 201 131 205 86 197 31zm717-1280v512h-128V859l-339 338-90-90 338-339h-293V640h512zm-640 0H896V128h128v293l339-338 90 90-338 339h293v128z" />
            </svg>
        }
    }
}
