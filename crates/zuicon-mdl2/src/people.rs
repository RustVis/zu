// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct People {}

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

impl Component for People {
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
                data-icon={ "People" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1636 693q65 33 117 81t90 108 57 128 20 142h-128q0-79-30-149t-83-122-122-82-149-31q-79 0-149 30t-122 83-82 122-31 149q0 91-41 173t-115 136q65 33 117 81t90 108 57 128 20 142h-128q0-79-30-149t-83-122-122-82-149-31q-79 0-149 30t-122 83-82 122-31 149H128q0-73 20-141t57-128 89-108 118-82q-73-54-114-136t-42-173q0-79 30-149t83-122 122-82 149-31q91 0 173 41t136 115q38-75 97-134t134-97q-73-54-114-136t-42-173q0-79 30-149t83-122 122-82 149-31q79 0 149 30t122 83 82 122 31 149q0 91-41 173t-115 136zm-996 715q52 0 99-20t81-55 55-81 21-100q0-52-20-99t-55-81-82-55-99-21q-53 0-99 20t-81 55-55 82-21 99q0 53 20 99t55 81 81 55 100 21zm512-1024q0 53 20 99t55 81 81 55 100 21q52 0 99-20t81-55 55-81 21-100q0-52-20-99t-55-81-82-55-99-21q-53 0-99 20t-81 55-55 82-21 99z" />
            </svg>
        }
    }
}
