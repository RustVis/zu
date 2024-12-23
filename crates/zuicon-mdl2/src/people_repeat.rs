// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PeopleRepeat {}

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

impl Component for PeopleRepeat {
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
                data-icon={ "PeopleRepeat" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 1152v384h-384v-128h153q-41-62-108-95t-141-33q-54 0-106 18t-92 52-65 81-25 105h-128q0-85 34-155t93-122 133-79 156-28q52 0 102 11t96 35 85 56 69 77v-179h128zm-416 768q54 0 105-18t92-52 66-81 25-105h128q0 85-34 155t-93 122-133 79-156 28q-52 0-102-11t-96-35-85-56-69-77v179h-128v-384h384v128h-153q41 62 108 95t141 33zM1408 768q-79 0-149 30t-122 82-82 122-31 149q0 93-41 174t-117 137q45 23 85 54t73 69l-102 79q-54-61-127-94t-155-34q-80 0-149 30t-122 82-83 123-30 149H128q0-73 20-141t57-129 90-108 118-81q-74-54-115-135t-42-174q0-79 30-149t82-122 122-83 150-30q92 0 173 41t136 116q38-75 97-134t135-98q-74-54-115-135t-42-174q0-79 30-149t82-122 122-83 150-30q79 0 149 30t122 82 83 123 30 149q0 92-41 173t-116 136q100 50 169 136t98 195h-134q-20-57-56-104t-83-81-103-52-118-19zm0-640q-53 0-99 20t-82 55-55 81-20 100q0 53 20 99t55 82 81 55 100 20q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20zM640 1408q53 0 99-20t82-55 55-81 20-100q0-53-20-100t-54-81-82-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 53 20 99t55 82 81 55 100 20z" />
            </svg>
        }
    }
}
