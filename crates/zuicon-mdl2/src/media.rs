// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Media {}

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

impl Component for Media {
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
                data-icon={ "Media" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 670v898q0 51-22 92t-59 70-82 46-93 16q-47 0-93-16t-82-45-58-71-23-92q0-51 22-92t59-71 82-45 93-16q66 0 128 31V834l-640 160v702q0 51-22 92t-59 70-82 46-93 16q-47 0-93-16t-82-45-58-71-23-92q0-51 22-92t59-71 82-45 93-16q66 0 128 31V894l896-224zM1024 1792q20 0 42-6t42-18 31-30 13-42q0-24-12-42t-32-30-41-18-43-6q-20 0-42 6t-42 18-31 30-13 42q0 24 12 42t32 30 41 18 43 6zm768-128q20 0 42-6t42-18 31-30 13-42q0-24-12-42t-32-30-41-18-43-6q-20 0-42 6t-42 18-31 30-13 42q0 24 12 42t32 30 41 18 43 6zM384 640H256V512h128v128zM256 768h128v128H256V768zm896-256h128v128h-128V512zm-128 768H0V128h1536v512l-128 32V256h-128v128h-128V256H384v128H256V256H128v896h128v-128h128v128h640v128z" />
            </svg>
        }
    }
}
