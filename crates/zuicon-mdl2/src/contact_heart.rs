// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ContactHeart {}

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

impl Component for ContactHeart {
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
                data-icon={ "ContactHeart" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 1484q0 54-20 102t-58 87l-370 369-370-369q-38-38-58-86t-20-103q0-55 21-104t57-85 84-58 105-21q50 0 97 17t84 53q38-35 84-52t97-18q56 0 104 21t85 57 57 86 21 104zm-128 0q0-29-11-54t-30-45-44-30-55-11q-57 0-98 41l-82 82-82-82q-20-20-45-30t-53-11q-29 0-54 11t-45 30-30 44-11 55q0 57 41 98l279 279 279-279q41-41 41-98zm-768-332q-87-65-181-96t-203-32q-134 0-251 49t-203 136-136 204-50 251H0q0-121 35-232t100-206 156-166 206-115q-55-34-99-82t-76-104-49-119-17-128q0-106 40-199t110-162T569 41 768 0q106 0 199 40t162 110 110 163 41 199q0 65-17 127t-48 119-76 105-100 82q66 23 121 58t109 82q-33 11-61 28t-56 39zm0-640q0-79-30-148t-83-122-122-83-149-31q-79 0-148 30t-122 83-83 122-31 149q0 79 30 148t83 122 122 83 149 31q79 0 148-30t122-83 83-122 31-149z" />
            </svg>
        }
    }
}
