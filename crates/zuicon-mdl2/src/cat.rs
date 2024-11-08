// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Cat {}

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

impl Component for Cat {
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
                data-icon={ "Cat" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 576q0 40-15 75t-41 61-61 41-75 15h-64v640q0 66 11 131t34 129q46 8 84 31t67 56 44 76 16 89v40q0 22-4 42t-18 33-42 13H512q-106 0-199-40t-162-110-110-163-41-199q0-79 30-149t82-122 122-83 150-30q26 0 49-10t41-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10V768q53 0 99 20t82 55 55 81 20 100q0 53-20 99t-55 82-81 55-100 20q-53 0-99 20t-82 55-55 81-20 100q0 80 30 149t82 122 122 83 150 30q26 0 49-10t41-27 28-41 10-50q0-198 69-369t205-315q117-124 177-274t61-322V384q0-79 30-149t82-122 122-83 150-30h128v113q0 17 9 33t26 24q61 30 121 60t122 61q49 25 77 71t29 101v113zm-128 1344q0-26-10-49t-27-41-41-28-50-10q-21 0-35-10t-24-29q-18-34-31-78t-21-90-13-93-4-84V768q0-27 10-50t27-40 41-28 50-10h40q22 0 42-4t33-18 13-42V463q0-17-9-33t-26-24q-61-30-121-60t-122-61q-45-23-72-64t-32-93q-53 0-100 20t-82 54-55 81-21 101v128q0 197-69 369t-205 315q-117 124-177 274t-61 322q0 34-9 66t-27 62h804q0-26-10-49t-27-41-41-28-50-10h-256v-128h256q8 0 15 1t16 2l-93-371 124-32 119 475q36 36 55 83t20 98h256zM1600 384q26 0 45 19t19 45q0 26-19 45t-45 19q-26 0-45-19t-19-45q0-26 19-45t45-19z" />
            </svg>
        }
    }
}
