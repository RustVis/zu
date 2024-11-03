// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct FieldString {}

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

impl Component for FieldString {
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
                data-icon={ "field-string" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <defs><style type="text/css"></style></defs><path d="M875.6 515.9c2.1 0.8 4.4-0.3 5.2-2.4 0.2-0.4 0.2-0.9 0.2-1.4v-58.3c0-1.8-1.1-3.3-2.8-3.8-6-1.8-17.2-3-27.2-3-32.9 0-61.7 16.7-73.5 41.2v-28.6c0-4.4-3.6-8-8-8H717c-4.4 0-8 3.6-8 8V729c0 4.4 3.6 8 8 8h54.8c4.4 0 8-3.6 8-8V572.7c0-36.2 26.1-60.2 65.1-60.2 10.4 0.1 26.6 1.8 30.7 3.4zM338.6 475.4l-54.7-12.6c-61.2-14.2-87.7-34.8-87.7-70.7 0-44.6 39.1-73.5 96.9-73.5 52.8 0 91.4 26.5 99.9 68.9h70C455.9 311.6 387.6 259 293.4 259c-103.3 0-171 55.5-171 139 0 68.6 38.6 109.5 122.2 128.5l61.6 14.3c63.6 14.9 91.6 37.1 91.6 75.1 0 44.1-43.5 75.2-102.5 75.2-60.6 0-104.5-27.2-112.8-70.5H111c7.2 79.9 75.6 130.4 179.1 130.4C402.3 751 471 695.2 471 605.3c0-70.2-38.6-108.5-132.4-129.9z" p-id="12119"></path><path d="M877 729m-36 0a36 36 0 1 0 72 0 36 36 0 1 0-72 0Z" p-id="12120"></path><path d="M653 457.8h-51.4V396c0-4.4-3.6-8-8-8h-54.7c-4.4 0-8 3.6-8 8v61.8H495c-4.4 0-8 3.6-8 8v42.3c0 4.4 3.6 8 8 8h35.9v147.5c0 56.2 27.4 79.4 93.1 79.4 11.7 0 23.6-1.2 33.8-3.1 1.9-0.3 3.2-2 3.2-3.9v-49.3c0-2.2-1.8-4-4-4h-0.4c-4.9 0.5-6.2 0.6-8.3 0.8-4.1 0.3-7.8 0.5-12.6 0.5-24.1 0-34.1-10.3-34.1-35.6V516.1H653c4.4 0 8-3.6 8-8v-42.3c0-4.4-3.6-8-8-8z" p-id="12121"></path>
            </svg>
        }
    }
}
