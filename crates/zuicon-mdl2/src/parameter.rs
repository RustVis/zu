// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Parameter {}

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

impl Component for Parameter {
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
                data-icon={ "Parameter" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1027 448q121 0 227 37t185 107 124 170 46 226q0 66-17 137t-54 130-93 97-135 38q-86 0-124-47t-39-129h-4q-23 79-74 127t-138 49q-59 0-101-23t-70-62-41-88-13-103q0-70 18-140t57-125 96-89 139-35q23 0 46 6t45 18 37 30 24 42v-5q2-20 3-40t4-40h93l-24 286q-3 32-7 65t-4 65q0 22 2 48t12 50 26 39 48 16q36 0 64-15t48-41 35-57 23-67 12-69 4-62q0-110-36-196t-100-145-154-90-196-31q-115 0-208 43T653 691 550 863t-36 209q0 115 35 208t102 160 159 102 209 36q81 0 160-13t153-48v92q-75 32-157 43t-163 12q-133 0-243-41t-189-118-124-186-44-243q0-135 45-250t126-199 194-131 250-48zm-78 855q61 0 100-34t61-86 30-109 9-105q0-34-7-63t-22-50-42-33-63-12q-58 0-97 30t-63 75-34 98-11 101q0 33 7 66t23 61 43 44 66 17zM0 256h384v128H128v1280h256v128H0V256zm2048 0v1536h-384v-128h256V384h-256V256h384z" />
            </svg>
        }
    }
}
