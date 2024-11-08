// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Calories {}

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

impl Component for Calories {
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
                data-icon={ "Calories" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1280 64q0 179 66 330t190 278 190 278 66 330q0 106-27 204t-78 183-120 156-155 120-184 77-204 28q-106 0-204-27t-183-78-156-120-120-155-77-184-28-204q0-84 18-165t52-155 84-141 113-121q7 38 19 78t28 80 38 76 46 67q20 25 52 25 27 0 45-19t19-46q0-11-3-20t-10-18q-28-41-49-81t-37-82-23-87-8-95q0-119 45-224t124-183T992 46t224-46h64v64zm-256 1856q133 0 249-50t204-137 137-203 50-250q0-151-56-281t-162-236q-130-131-204-289t-88-342q-83 11-153 50t-123 99-81 135-29 160q0 78 23 141t68 126q19 26 29 54t11 62q0 40-15 75t-42 61-61 42-75 15q-46 0-81-17t-62-46-48-65-40-72q-46 73-68 157t-23 171q0 133 50 249t137 204 203 137 250 50z" />
            </svg>
        }
    }
}
