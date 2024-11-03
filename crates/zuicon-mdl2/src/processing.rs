// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Processing {}

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

impl Component for Processing {
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
                data-icon={ "Processing" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M837 844q-23 37-53 67t-68 54l51 124-118 48-51-123q-40 10-86 10t-86-10l-51 123-118-48 51-124q-37-23-67-53t-54-68L63 895 15 777l123-51q-10-40-10-86t10-86L15 503l48-118 124 51q46-75 121-121l-51-124 118-48 51 123q40-10 86-10t86 10l51-123 118 48-51 124q75 46 121 121l124-51 48 118-123 51q10 40 10 86t-10 86l123 51-48 118-124-51zm-325 52q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 53 20 99t55 82 81 55 100 20zm1408 448q0 55-14 111l137 56-48 119-138-57q-59 98-156 156l57 137-119 49-56-137q-56 14-111 14t-111-14l-56 137-119-49 57-137q-98-58-156-156l-138 57-48-119 137-56q-14-56-14-111t14-111l-137-56 48-119 138 57q58-97 156-156l-57-138 119-48 56 137q56-14 111-14t111 14l56-137 119 48-57 138q97 59 156 156l138-57 48 119-137 56q14 56 14 111zm-448 320q66 0 124-25t101-68 69-102 26-125q0-66-25-124t-69-101-102-69-124-26q-66 0-124 25t-102 69-69 102-25 124q0 66 25 124t68 102 102 69 125 25z" />
            </svg>
        }
    }
}
