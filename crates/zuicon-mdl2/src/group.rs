// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Group {}

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

impl Component for Group {
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
                data-icon={ "Group" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1850 688q45 25 82 61t62 80 40 93 14 102h-128q0-52-20-99t-55-81-82-55-99-21q-53 0-99 20t-81 55-55 82-21 99q0 91-41 173t-115 136q65 33 117 81t90 108 57 128 20 142h-128q0-79-30-149t-83-122-122-82-149-31q-79 0-149 30t-122 83-82 122-31 149H512q0-73 20-141t57-128 89-108 118-82q-73-54-114-136t-42-173q0-52-20-99t-55-81-82-55-99-21q-53 0-99 20t-81 55-55 82-21 99H0q0-52 14-101t39-93 63-80 82-62q-33-35-51-81t-19-95q0-52 20-99t55-81 81-55 100-21q52 0 99 20t81 55 55 82 21 99q0 49-18 95t-52 81q82 45 134 124 54-80 138-126t182-46q97 0 181 46t139 126q52-79 134-124-33-35-51-81t-19-95q0-52 20-99t55-81 81-55 100-21q52 0 99 20t81 55 55 82 21 99q0 49-18 95t-52 81zM256 512q0 27 10 50t27 40 41 28 50 10q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50zm768 768q52 0 99-20t81-55 55-81 21-100q0-52-20-99t-55-81-82-55-99-21q-53 0-99 20t-81 55-55 82-21 99q0 53 20 99t55 81 81 55 100 21zm512-768q0 27 10 50t27 40 41 28 50 10q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50z" />
            </svg>
        }
    }
}
