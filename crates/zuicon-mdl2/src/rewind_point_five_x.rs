// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct RewindPointFiveX {}

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

impl Component for RewindPointFiveX {
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
                data-icon={ "RewindPointFiveX" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 513v517L380 510 1024-5v512L1664-5v1035l-640-517zm512-252l-312 250 312 251V261zM896 762V261L584 511l312 251zm1152 646l-215 324 211 316h-119q-39-64-77-127t-77-129h-2q-42 63-80 127t-78 129h-118l218-314-209-326h120q38 67 77 133t73 136h3l160-269h113zM703 1856q19 0 37 7t31 21 21 31 8 37q0 41-28 68t-69 28q-40 0-67-28t-28-68q0-19 7-36t21-31 30-21 37-8zm688-92q0 69-24 122t-67 89-99 54-122 19q-22 0-46-1t-48-5-47-11-42-20v-107q42 27 88 42t96 16q44 0 82-13t66-37 44-60 16-82q0-51-18-86t-50-56-72-32-86-10h-27q-30 0-61 1t-61 7l30-442h407v91h-320l-17 258q20 0 40-1t40-2q63 0 117 16t95 50 63 83 23 117zM292 1152q59 0 102 20t74 54 49 78 29 94 14 101 4 97q0 47-5 99t-17 102-33 95-53 80-77 55-105 21q-57 0-99-19t-73-53-50-76-31-92-16-97-5-94q0-48 4-101t16-105 33-99 53-82 78-57 108-21zm-9 811q42 0 71-18t50-48 32-67 18-78 7-78 2-68q0-29-1-67t-7-80-17-81-31-71-49-51-71-20q-44 0-74 19t-51 51-33 72-18 82-7 81-2 71q0 30 1 68t8 77 18 76 32 66 50 46 72 18z" />
            </svg>
        }
    }
}
