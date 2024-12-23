// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Flower {}

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

impl Component for Flower {
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
                data-icon={ "Flower" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1600 576q0 40-15 75t-41 61-61 41-75 15h-116q13 23 27 45t27 46 20 49 8 52q0 39-15 74t-41 61-61 42-75 15q-48 0-89-22t-69-61v425q41-51 91-91t108-67 121-41 128-15h64v64q0 110-39 208t-108 176-162 126-203 63v131H896v-131q-109-13-202-62t-163-127-108-175-39-209v-64h64q65 0 128 14t120 42 108 67 92 91v-425q-27 39-68 61t-90 22q-40 0-75-15t-61-41-41-62-15-74q0-27 8-52t20-48 26-46 28-46H512q-40 0-75-15t-61-41-41-61-15-75q0-40 15-75t41-61 61-41 75-15h116q-13-22-27-45t-27-46-20-49-8-52q0-39 15-74t41-61 61-42 75-15q43 0 74 13t55 37 41 53 37 63l15 26 15-26q18-33 36-63t42-53 54-36 75-14q40 0 75 15t61 41 41 62 15 74q0 27-8 52t-20 48-26 47-28 45h116q40 0 75 15t61 41 41 61 15 75zm-419-448q-17 0-32 9t-23 23l-97 169q63 18 110 64l98-169q9-15 9-32 0-26-19-45t-46-19zM960 704q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10zM739 128q-27 0-46 19t-19 45q0 17 9 32l98 169q47-46 110-64l-97-169q-8-14-23-23t-32-9zm-27 512q-4-16-6-31t-2-33q0-17 2-32t6-32H512q-26 0-45 19t-19 45q0 26 19 45t45 19h200zm27 384q17 0 32-8t23-24l97-169q-63-18-110-64l-98 169q-9 15-9 32 0 26 19 45t46 19zm-222 389q11 72 44 135t82 112 113 83 135 44q-11-72-44-135t-82-112-113-83-135-44zm885 0q-72 11-135 44t-112 82-82 113-44 135q72-11 135-44t112-82 82-113 44-135zm-221-389q27 0 46-19t19-45q0-17-9-32l-98-169q-47 46-110 64l97 169q8 15 23 23t32 9zm227-384q26 0 45-19t19-45q0-26-19-45t-45-19h-200q4 16 6 31t2 33q0 17-2 32t-6 32h200z" />
            </svg>
        }
    }
}
