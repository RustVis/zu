// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct RewindOneFiveX {}

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

impl Component for RewindOneFiveX {
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
                data-icon={ "RewindOneFiveX" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 513v517L380 510 1024-5v512L1664-5v1035l-640-517zm512-252l-312 250 312 251V261zM896 762V261L584 511l312 251zm-550 390h38v896H284v-757q-19 17-44 33t-52 29-54 25-53 17v-102q30-8 67-23t73-34 69-41 56-43zm229 704q19 0 37 7t31 21 21 31 8 37q0 41-28 68t-69 28q-40 0-67-28t-28-68q0-19 7-36t21-31 30-21 37-8zm712-92q0 69-24 122t-67 89-99 54-122 19q-22 0-46-1t-48-5-46-11-42-20v-107q42 27 87 42t96 16q44 0 82-12t66-37 44-61 16-82q0-51-18-86t-50-57-72-31-86-10q-36 0-75 1t-74 7l30-442h408v91H927l-18 258q20 0 40-1t41-2q63 0 117 16t95 50 62 83 23 117zm660-356l-215 324 211 316h-119q-39-64-77-127t-77-129h-2q-42 63-80 127t-78 129h-118l218-314-209-326h120q38 67 76 133t75 136h2l160-269h113z" />
            </svg>
        }
    }
}
