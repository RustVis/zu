// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ManagerSelfService {}

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

impl Component for ManagerSelfService {
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
                data-icon={ "ManagerSelfService" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 1792h-227q48 53 73 118t26 138h-128q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100h-128q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100H512q0-52 14-102t39-93 63-80 83-61q-34-35-52-81t-19-95q0-69 34-127t94-93v-292q0-30 13-57t38-45q-55-73-135-113t-172-41q-80 0-149 30t-122 82-83 123-30 149H0q0-73 20-141t57-129 91-108 118-81q-75-54-116-135t-42-174q0-79 30-149t82-122 122-83T512 0q79 0 149 30t122 82 83 123 30 149q0 93-41 174T738 694q68 34 123 85t94 117h357q31 0 54-9t43-24 41-31 46-31 58-23 78-10h288q27 0 50 10t40 27 28 41 10 50v896zM512 640q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 53 20 99t55 82 81 55 100 20zm384 1024q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10zm640 0q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10zm384-512h-288q-45 0-78-9t-58-24-45-31-41-31-44-23-54-10H896v256q53 0 99 20t82 55 55 81 20 100q0 49-18 95t-53 81q83 46 135 124 52-78 135-124-34-35-52-81t-19-95q0-53 20-99t55-82 81-55 100-20q53 0 99 20t82 55 55 81 20 100q0 34-9 66t-27 62h164v-512zm0-256h-288q-23 0-41 5t-34 13-31 20-32 26q16 14 31 25t32 20 34 14 41 5h288V896z" />
            </svg>
        }
    }
}
