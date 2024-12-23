// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ExerciseTracker {}

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

impl Component for ExerciseTracker {
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
                data-icon={ "ExerciseTracker" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M128 640q42 0 67 20t43 44 37 44 45 20q48 0 85-19t68-47 61-62 64-61 74-48 96-19q36 0 70 11t63 32l1 2q17 12 32 24t33 25q12-12 25-21t32-9q27 0 45 19t19 46q0 20-13 38 32 20 64 38t64 37q16-17 34-33t43-17q26 0 45 19t19 45q0 16-6 26t-15 22q34 17 68 33t70 31q16-16 33-32t42-16q26 0 45 19t19 45q0 21-12 37 63 26 127 48t129 46q34 13 70 33t66 49 49 63 19 76v32q0 56-27 101t-70 80-92 62-96 43q-145 55-300 76t-311 22h-128q-42 0-75-10t-63-22-57-22-61-10q-27 0-50 6t-48 19q-38 19-76 29t-82 10H192q-40 0-75-15t-61-41-41-61-15-75V768q0-27 10-50t27-40 41-28 50-10zm1792 608q0-16-13-31t-33-29-39-24-34-16q-78-30-155-57t-155-59l-102 101q-19 19-45 19t-45-19-19-45q0-15 9-30t23-30 27-28 25-23q-35-16-69-32t-68-34l-94 94q-19 19-45 19t-45-19-19-45q0-26 19-45l66-67q-32-18-63-37t-63-39L877 877q-19 19-45 19t-45-19-19-45q0-26 19-45l89-89q-24-19-49-38t-59-20q-28 0-53 12t-44 32h-1q-40 40-76 78t-76 68-87 48-111 18q-32 0-58-9t-47-25-41-37-36-45q-6-8-10-11v511h1792v-32zM512 1536q33 0 60-10t57-22 62-22 77-10q44 0 81 10t77 29q35 17 67 22t71 5q87 0 184-4t195-18 190-39 169-69H128v64q0 26 19 45t45 19h320z" />
            </svg>
        }
    }
}
