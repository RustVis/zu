// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct FieldNumber {}

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

impl Component for FieldNumber {
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
                data-icon={ "field-number" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <defs><style type="text/css"></style></defs><path d="M508 280h-63.3c-3.3 0-6 2.7-6 6v340.2H433L197.4 282.6c-1.1-1.6-3-2.6-4.9-2.6H126c-3.3 0-6 2.7-6 6v464c0 3.3 2.7 6 6 6h62.7c3.3 0 6-2.7 6-6V405.1h5.7l238.2 348.3c1.1 1.6 3 2.6 5 2.6H508c3.3 0 6-2.7 6-6V286c0-3.3-2.7-6-6-6zM886 693H582c-4.4 0-8 3.6-8 8v48c0 4.4 3.6 8 8 8h304c4.4 0 8-3.6 8-8v-48c0-4.4-3.6-8-8-8zM733.8 630c52.9 0 95.2-17.2 126.2-51.7 29.4-32.9 44-75.8 44-128.8 0-53.1-14.6-96.5-44-129.3-30.9-34.8-73.2-52.2-126.2-52.2-53.7 0-95.9 17.5-126.3 52.8-29.2 33.1-43.4 75.9-43.4 128.7 0 52.4 14.3 95.2 43.5 128.3 30.6 34.7 73 52.2 126.2 52.2z m-71.5-263.7c16.9-20.6 40.3-30.9 71.4-30.9 31.5 0 54.8 9.6 71 29.1 16.4 20.3 24.9 48.6 24.9 84.9 0 36.3-8.4 64.1-24.8 83.9-16.5 19.4-40 29.2-71.1 29.2-31.2 0-55-10.3-71.4-30.4-16.3-20.1-24.5-47.3-24.5-82.6 0.1-35.8 8.2-63 24.5-83.2z" p-id="12001"></path>
            </svg>
        }
    }
}
