// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Bug {}

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

impl Component for Bug {
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
                data-icon={ "Bug" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1608 897q65 2 122 27t99 69 66 100 25 123v192h-128v-192q0-32-10-61t-29-54-45-42-57-27q6 29 9 59t4 61v256q0 7-1 13t-2 13l6-6q60 60 92 138t32 163q0 85-32 162t-92 138l-90-90q42-42 64-95t22-114q0-68-31-132-31 100-90 183t-140 142-176 92-202 33q-105 0-201-33t-177-92-139-142-91-183q-31 64-31 132 0 60 22 113t64 96l-90 90q-60-60-92-137t-33-163q0-85 32-163t93-138l6 6q-1-7-2-13t-1-13v-256q0-30 3-60t10-60q-31 9-57 26t-44 42-29 54-11 62v192H128v-192q0-65 24-122t67-101 99-68 122-28q31-70 80-135-57-10-105-38t-84-71-55-94-20-111V256h128v192q0 40 15 75t41 61 61 41 75 15h64v3q47-35 96-59-15-32-23-66t-9-70q0-70 31-135L595 173l90-90 127 127q45-39 98-60t114-22q60 0 113 21t99 61l127-127 90 90-140 140q31 65 31 135 0 35-8 69t-24 67q26 13 49 27t47 32v-3h64q40 0 75-15t61-41 41-61 15-75V256h128v192q0 58-20 110t-55 95-83 70-106 39q49 65 80 135zm-584-641q-40 0-75 15t-61 41-41 61-15 75q0 50 24 90 42-11 83-17t85-7q43 0 84 6t84 18q24-40 24-90 0-40-15-75t-41-61-61-41-75-15zm512 896q0-104-41-197t-110-163-163-111-198-41q-105 0-198 41T664 792 553 955t-41 197v256q0 106 40 199t110 162 163 110 199 41q106 0 199-40t162-110 110-163 41-199v-256z" />
            </svg>
        }
    }
}
