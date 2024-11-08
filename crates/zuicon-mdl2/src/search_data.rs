// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SearchData {}

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

impl Component for SearchData {
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
                data-icon={ "SearchData" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1664 896q79 0 149 30t122 82 83 123 30 149q0 80-30 149t-82 122-123 83-149 30q-60 0-117-18t-105-53l-437 436q-19 19-45 19t-45-19-19-45q0-26 19-45l436-437q-35-48-53-105t-18-117q0-79 30-149t82-122 122-83 150-30zm0 640q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 53 20 99t55 82 81 55 100 20zm-1280 64q0 15 8 27t19 23q29 29 73 51t95 37 100 26 89 16v128h-8q-6 0-8-1-39-5-90-15t-107-27-108-40-95-56-68-74-26-95V448q0-48 22-86t58-70 83-54 96-41 95-29 84-18q66-11 132-16t132-6q67 0 133 5t132 18q36 6 83 17t95 29 96 40 83 55 59 69 23 87v320h-128V637q-58 37-130 62t-148 40-153 22-145 7q-68 0-144-6t-153-22-149-41-130-62v963zM960 256q-57 0-130 6t-148 20-143 40-115 63q-14 11-27 27t-13 36q0 19 13 35t27 28q46 38 114 63t143 39 148 21 131 6q57 0 130-6t148-20 143-40 114-63q14-11 27-27t14-36q0-19-13-35t-28-28q-46-38-114-63t-142-39-148-21-131-6z" />
            </svg>
        }
    }
}
