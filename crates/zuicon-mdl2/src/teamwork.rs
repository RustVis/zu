// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Teamwork {}

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

impl Component for Teamwork {
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
                data-icon={ "Teamwork" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1664 1088q66 0 124 25t102 68 69 102 25 125q0 52-16 101t-48 91v424l-256-128-256 128v-424q-31-42-47-91t-17-101q0-66 25-124t68-102 102-69 125-25zm0 128q-40 0-75 15t-61 41-41 61-15 75q0 40 15 75t41 61 61 41 75 15q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15zm128 600v-115q-60 27-128 27t-128-27v115l128-64 128 64zM1664 512q-53 0-99 20t-82 55-55 81-20 100q0 92-41 173t-116 137q19 9 36 20t35 23l-75 104q-49-35-106-54t-117-19q-80 0-149 30t-122 82-83 123-30 149H512q0-73 20-141t57-128 89-108 118-82q-74-55-115-136t-41-173q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100H0q0-52 14-101t39-93 62-80 83-62q-33-35-51-81t-19-95q0-53 20-99t55-82 81-55T384 0q53 0 99 20t82 55 55 81 20 100q0 49-18 95t-52 81q82 45 134 124 27-40 62-72t76-54 87-34 95-12q48 0 94 12t87 34 77 54 62 72q52-79 134-124-33-35-51-81t-19-95q0-53 20-99t55-82 81-55 100-20q53 0 99 20t82 55 55 81 20 100q0 49-18 95t-52 81q46 26 82 62t62 79 40 93 14 102h-128q0-53-20-99t-55-82-81-55-100-20zm-128-256q0 27 10 50t27 40 41 28 50 10q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50zm-1280 0q0 27 10 50t27 40 41 28 50 10q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50zm512 512q0 53 20 99t55 82 81 55 100 20q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100z" />
            </svg>
        }
    }
}
