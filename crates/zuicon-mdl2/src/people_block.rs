// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PeopleBlock {}

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

impl Component for PeopleBlock {
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
                data-icon={ "PeopleBlock" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1408 768q-80 0-149 30t-122 82-83 123-30 149q0 92-41 173t-115 136q65 33 117 81t90 108 57 128 20 142h-128q0-79-30-149t-82-122-123-83-149-30q-80 0-149 30t-122 82-83 123-30 149H128q0-73 20-141t57-128 89-108 118-82q-74-55-115-136t-41-173q0-79 30-149t82-122 122-83 150-30q92 0 173 41t136 115q38-75 97-134t134-97q-74-55-115-136t-41-173q0-79 30-149t82-122 122-83 150-30q79 0 149 30t122 82 83 123 30 149q0 92-41 173t-115 136q65 33 117 81t90 108 57 128 20 142h-128q0-79-30-149t-82-122-123-83-149-30zm-256-384q0 53 20 99t55 82 81 55 100 20q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100zM640 1408q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 53 20 99t55 82 81 55 100 20zm1179 320l226 227-90 90-227-226-227 227-90-91 227-227-227-227 90-90 227 227 227-227 90 91-226 226z" />
            </svg>
        }
    }
}
