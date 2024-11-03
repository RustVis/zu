// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BackgroundColor {}

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

impl Component for BackgroundColor {
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
                data-icon={ "BackgroundColor" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 256v1792H256v-640l128 128v384h1536V384h-768V256h896zM704 1562L102 960l538-539V192q0-40 15-75t41-61 61-41 75-15q40 0 75 15t61 41 41 61 15 75v576H896V192q0-26-19-45t-45-19q-26 0-45 19t-19 45v283L347 896h842l65-64-83-83 90-90 173 173-730 730zm357-538H347l357 358 357-358zm417 786q-67 0-129-23t-111-64-77-100-29-129q0-43 12-84t35-77l293-461 286 450q23 38 36 79t13 87q0 67-26 126t-72 102-105 69-126 25zm-161-389q-22 33-22 71 0 36 15 64t41 49 57 30 65 11q33 0 64-11t54-33 38-50 15-64q0-41-24-79l-148-233-155 245z" />
            </svg>
        }
    }
}
