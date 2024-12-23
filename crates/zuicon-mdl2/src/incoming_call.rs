// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct IncomingCall {}

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

impl Component for IncomingCall {
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
                data-icon={ "IncomingCall" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1530 1307q39 0 76 15t65 43q21 21 47 44t55 49 54 54 47 57 33 62 13 66q0 39-15 76t-43 65q-51 51-92 90t-85 66-99 40-129 14q-109 0-224-34t-229-94-223-143-206-181-180-207-141-222-92-227-34-220q0-76 13-130t40-98 66-85 91-93q28-28 65-43t76-15q50 0 96 27t89 66 79 82 68 74q28 28 43 65t15 76q0 39-13 70t-32 55-42 45-42 40-33 37-13 40q0 12 18 38t50 62 71 77 84 85 87 85 81 78 66 63 42 40q21 21 50 21 21 0 39-13t37-32 39-42 46-42 55-33 71-13zm-73 613q61 0 102-11t75-34 64-54 73-73q21-21 21-50 0-13-16-37t-40-54-55-60-60-55-53-41-38-16q-21 0-40 13t-37 32-39 42-46 42-55 33-70 13q-40 0-76-15t-65-44l-477-477q-28-28-43-64t-16-77q0-39 13-69t32-56 42-45 42-40 33-37 13-40q0-13-16-37t-40-53-55-60-60-55-54-41-37-16q-30 0-51 21-41 41-73 72t-54 64-33 75-12 101q0 96 31 198t86 206 132 202 165 187 188 164 202 130 203 85 194 31zm335-896h-640V384h128v421l595-594 90 90-594 595h421v128z" />
            </svg>
        }
    }
}
