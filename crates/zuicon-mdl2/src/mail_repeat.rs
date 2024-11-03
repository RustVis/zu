// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct MailRepeat {}

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

impl Component for MailRepeat {
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
                data-icon={ "MailRepeat" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 1664q2 78-30 146t-86 119-124 83-144 35h-9l-2 1h-11q-109 0-204-43t-158-136v179h-128v-384h384v128h-152q19 29 46 52t58 41 66 26 69 9q56 0 105-14t87-44 65-71 38-99l1-2v-11l1-2v-13h128zm-128-512h128v384h-384v-128h152q-19-29-46-52t-58-41-66-26-69-9q-56 0-105 14t-87 44-65 71-38 99l-1 2v11l-1 2v13h-128q-2-78 29-146t86-119 124-83 145-35h9l2-1h11q109 0 204 43t158 136v-179zm128-768v640h-128V648l-896 447-896-447v888h896v128H0V384h2048zM1024 953l881-441H143l881 441z" />
            </svg>
        }
    }
}
