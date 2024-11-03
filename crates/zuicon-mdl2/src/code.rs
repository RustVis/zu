// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Code {}

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

impl Component for Code {
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
                data-icon={ "Code" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M128 896q52 0 99-20t81-55 55-81 21-100q0-71-3-142t4-138 32-131 79-117q54-54 125-83T768 0v128q-53 0-99 20t-81 55-55 82-21 99q0 56 2 110t1 107-10 102-27 95-52 86-85 76q52 35 85 76t52 86 27 94 9 102 0 107-2 111q0 53 20 99t55 81 81 55 100 21v128q-76 0-147-29t-125-83q-54-54-78-117t-32-130-5-139 3-142q0-52-20-99t-55-81-82-55-99-21V896zM1280 0q76 0 147 29t125 83q54 54 78 117t32 130 5 139-3 142q0 53 20 99t55 81 81 55 100 21v128q-53 0-99 20t-81 55-55 82-21 99q0 71 3 142t-4 138-32 131-79 117q-54 54-125 83t-147 29v-128q52 0 99-20t81-55 55-81 21-100q0-56-2-110t-1-107 10-102 27-95 52-86 85-76q-52-35-85-76t-52-86-27-94-9-102 0-107 2-111q0-52-20-99t-55-81-82-55-99-21V0z" />
            </svg>
        }
    }
}
