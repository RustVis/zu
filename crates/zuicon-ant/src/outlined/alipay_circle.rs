// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AlipayCircle {}

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

impl Component for AlipayCircle {
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
                data-icon={ "alipay-circle" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M512 64c247.424 0 448 200.576 448 448S759.424 960 512 960 64 759.424 64 512 264.576 64 512 64m32.493 168c-69.66 0-86.056 16.843-86.709 39.079l-.02 1.426v46.623H291.45c-9.92 0-14.28 23.053-14.27 39.31 0 2.696 2.08 4.923 4.77 4.923h175.814v58.301h-116.5c-9.96 0-14.3 23.76-14.271 39.473a4.772 4.772 0 0 0 4.77 4.76l233.448.003c-4.527 41.056-15.432 77.58-30.716 109.315l-1.224 2.494-.32-.275c-60.244-28.47-120.431-52.577-194.407-52.577l-2.61.017c-84.982 1.112-144.718 56.503-145.916 127.04l-.018 1.222.019 2.123c1.238 70.399 63.566 126.452 148.525 126.452 61.245-.008 116.372-16.85 163.457-45.017a138.579 138.579 0 0 0 14.068-7.962c18.09-12.116 34.892-25.955 50.304-41.156l9.452 6.344 12.456 8.322c57.527 38.257 113.763 72.617 169.856 79.27a142.62 142.62 0 0 0 18.314 1.157c43.017 0 54.991-52.68 57.387-95.508l.145-2.84c.392-8.463-6.197-15.595-14.648-15.863-75.468-2.365-136.452-22.043-192.008-46.11l-6.267-2.742c35.146-56.8 56.657-121.816 57.155-186.661l.082-1.083c.401-5.515-3.997-10.198-9.52-10.198H549.33v-58.301h165.732c9.92 0 14.28-22.117 14.27-39.311-.01-2.686-2.089-4.922-4.779-4.922H549.32v-82.35c0-2.656-2.175-4.778-4.827-4.778m-216.5 351.847c54.627 0 107.073 22.417 158.09 52.19l5.77 3.402c-103.575 119.837-247.172 95.903-261.724 26.37a66.89 66.89 0 0 1-1.138-9.83l-.057-2.336.013-.907c.969-40.113 45.337-68.89 99.045-68.89"/>
            </svg>
        }
    }
}
