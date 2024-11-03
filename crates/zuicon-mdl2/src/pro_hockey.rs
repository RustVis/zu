// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ProHockey {}

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

impl Component for ProHockey {
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
                data-icon={ "ProHockey" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1324 0q78 0 149 23t128 79l345 345q56 56 79 127t23 150q0 104-34 216t-95 224-144 222-181 208-207 181-222 144-225 95-216 34q-77 0-149-23t-128-79l-345-345q-56-56-79-127T0 1324q0-104 34-216t94-225 144-222 182-207 207-181 222-144 225-95 216-34zM128 1327q0 121 65 184t186 63q90 0 189-31t201-88 200-132 188-165 166-189 132-200 87-200 32-190q0-121-64-186t-185-65q-91 0-190 31t-201 88-201 132-188 166-166 189-132 201-87 201-32 191zm593 593q91 0 191-31t201-88 201-132 189-165 166-189 132-201 87-202 32-191q0-51-14-98t-51-85l-152-153q-2 103-36 214t-95 223-144 220-180 205-206 181-219 143-223 96-215 36l153 152q37 36 84 50t99 15z" />
            </svg>
        }
    }
}
