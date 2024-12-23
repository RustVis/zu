// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct GlobeFavorite {}

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

impl Component for GlobeFavorite {
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
                data-icon={ "GlobeFavorite" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2031 1537l-266 197 102 313-267-193-266 194 101-314-266-197h335l97-310 95 310h335zM679 1408q8 37 23 89t37 109 51 113 64 101 78 72 92 28q36 0 69-17t59-41v178q-32 4-64 6t-64 2q-141 0-272-36t-244-104-207-160-161-207-103-245-37-272q0-141 36-272t104-244 160-207 207-161T752 37t272-37q141 0 272 36t244 104 207 160 161 207 103 245 37 272q0 99-19 195t-56 189h-181v-128h91q37-123 37-256t-37-256h-363q1 12 5 48t7 76 4 74 0 41l-126 401H679zm712-128q9-64 13-127t4-129q0-65-4-128t-13-128H657q-9 64-13 127t-4 129q0 65 4 128t13 128h734zm442-640q-38-81-92-153t-120-131-143-104-163-75q36 49 65 105t52 116 39 121 29 121h333zm-809-512q-49 0-91 27t-78 73-65 101-51 113-37 109-23 89h690q-8-37-23-89t-37-109-51-113-64-101-78-72-92-28zm-291 49q-85 29-162 74T427 356 307 487t-92 153h333q12-60 28-121t39-121 52-116 66-105zm-605 847q0 133 37 256h363q-8-64-12-127t-4-129q0-65 4-128t12-128H165q-37 123-37 256zm87 384q38 81 92 153t120 131 143 104 163 75q-36-49-65-105t-52-116-39-121-29-121H215z" />
            </svg>
        }
    }
}
