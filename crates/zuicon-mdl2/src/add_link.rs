// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AddLink {}

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

impl Component for AddLink {
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
                data-icon={ "AddLink" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M768 1216q0 66 25 124t68 102 102 69 125 25h192v128h-192q-93 0-174-35t-142-96-96-142-36-175q0-93 35-174t96-142 142-96 175-36h64v128h-64q-66 0-124 25t-102 69-69 102-25 124zm768-320V768h64q93 0 174 35t142 96 96 142 36 175q0 88-33 169t-95 144v-313q0-66-25-124t-69-101-102-69-124-26h-64zM960 384q93 0 174 35t142 96 96 142 36 175q0 93-35 174t-96 142-142 96-175 36h-64v-128h64q66 0 124-25t101-68 69-102 26-125q0-66-25-124t-69-101-102-69-124-26H448q-66 0-124 25t-102 69-69 102-25 124q0 66 25 124t68 102 102 69 125 25h64v128h-64q-93 0-174-35t-142-96-96-142T0 832q0-93 35-174t96-142 142-96 175-36h512zm1088 1280v128h-256v256h-128v-256h-256v-128h256v-256h128v256h256z" />
            </svg>
        }
    }
}
