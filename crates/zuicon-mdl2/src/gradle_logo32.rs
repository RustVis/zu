// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct GradleLogo32 {}

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

impl Component for GradleLogo32 {
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
                data-icon={ "GradleLogo32" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 576q0 64-15 129t-40 130-56 124-64 115q-31 52-63 96t-64 85-59 84-49 88-33 101-13 122q0 36 3 71t11 71h-245q-29-37-70-80t-90-80-99-61-101-25q-49 0-86 23t-64 60-45 79-31 84H587q-4-30-14-66t-30-68-46-53-65-22q-38 0-66 21t-50 53-34 68-20 67H17q-10-55-13-111t-4-113q0-71 7-145t24-145 45-137 70-122 99-101 132-71q22 51 47 100t56 95q25 39 64 63t87 25q44 0 84-20t75-50 65-66 52-70q-23 25-54 53t-68 51-75 39-72 16q-39 0-65-18t-47-50q-9-15-24-38t-31-52-33-59-30-58-21-51-9-38q0-34 28-61t73-47 100-35 108-24 98-13 70-5q139 0 258 41t233 119q51 35 106 57t118 23q54 0 107-22t96-59 69-88 27-108q0-36-14-67t-38-54-55-37-67-14q-23 0-41 5t-34 12-29 12-23 6q-16 0-25-13-3-4-9-15t-13-25-12-26-5-18q0-24 22-41t51-28 60-16 48-5q85 0 152 32t115 88 71 129 25 154zm-783 378q0 29 11 52t29 41 44 26 54 9q19 0 41-7t41-19 34-30 19-37q-17-6-35-12t-35-11q1 5 2 8t1 8q0 29-20 47t-49 19q-27 0-47-18t-20-46q0-16 7-29t21-23l-90-29q-5 12-6 25t-2 26z" />
            </svg>
        }
    }
}
