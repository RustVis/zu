// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct LinkedDatabase {}

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

impl Component for LinkedDatabase {
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
                data-icon={ "LinkedDatabase" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M128 1600q0 20 13 35t27 28q35 29 81 50t98 35 107 23 111 13 107 6 96 2v128h-74q-47 0-116-6t-147-20-154-37-138-59-100-85-39-113V448q0-69 39-119t103-86 142-58 157-35 148-17 115-5q45 0 115 4t148 17 157 35 142 58 102 86 40 120v576h-128V637q-58 37-130 62t-148 40-154 22-144 7q-68 0-144-6t-153-22-149-41-130-62v963zM704 256q-59 0-132 6t-148 20-142 40-114 63q-14 12-27 27t-13 36q0 20 13 35t27 28q46 38 114 63t142 39 147 21 133 6q58 0 131-6t148-20 142-40 114-63q14-11 27-27t14-36q0-20-13-36t-28-27q-47-38-114-63t-141-39-148-21-132-6zm384 1024q-40 0-75 15t-61 41-41 61-15 75q0 65 37 113t97 70q-6 36-6 73 0 15 1 29t3 29q-56-9-104-38t-82-71-54-96-20-109q0-66 25-124t68-102 102-69 125-25h256q66 0 124 25t101 69 69 102 26 124q0 54-20 105t-56 93-81 73-99 43v-133q42-9 67-23t38-36 18-52 5-70q0-40-15-75t-41-61-61-41-75-15h-256zm700 134q57 12 104 40t82 70 54 93 20 111q0 66-25 124t-68 102-102 69-125 25h-256q-67 0-125-25t-101-68-69-102-25-125q0-57 19-109t53-93 81-71 103-41v133q-58 21-93 69t-35 112q0 40 15 75t41 61 61 41 75 15h256q40 0 75-15t61-41 41-61 15-75q0-65-37-113t-97-70q6-36 6-73 0-15-1-29t-3-29z" />
            </svg>
        }
    }
}
