// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AlipaySquare {}

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

impl Component for AlipaySquare {
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
                data-icon={ "alipay-square" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M894.606 116.541c5.541 2.963 9.89 7.312 12.853 12.853 2.963 5.54 4.541 11.036 4.541 26.192v712.828c0 15.156-1.578 20.652-4.541 26.192-2.963 5.541-7.312 9.89-12.853 12.853-5.54 2.963-11.036 4.541-26.192 4.541H155.586c-15.156 0-20.652-1.578-26.192-4.541-5.541-2.963-9.89-7.312-12.853-12.853-2.923-5.465-4.498-10.888-4.54-25.583L112 155.586c0-15.156 1.578-20.652 4.541-26.192 2.963-5.541 7.312-9.89 12.853-12.853 5.465-2.923 10.888-4.498 25.583-4.54L868.414 112c15.156 0 20.652 1.578 26.192 4.541M541.012 262c-62.197 0-76.836 15.038-77.419 34.892l-.018 1.274v41.627H315.08c-8.858 0-12.75 20.583-12.742 35.098 0 2.407 1.857 4.395 4.259 4.395h156.977v52.055H359.556c-8.891 0-12.767 21.215-12.741 35.244a4.26 4.26 0 0 0 4.259 4.25l208.436.003c-4.042 36.657-13.779 69.267-27.425 97.602l-1.093 2.227-.285-.245c-53.79-25.421-107.528-46.944-173.578-46.944l-2.33.015c-75.878.993-129.213 50.449-130.283 113.43l-.016 1.09.017 1.895c1.105 62.856 56.756 112.904 132.612 112.904 54.682-.008 103.903-15.046 145.943-40.194a123.73 123.73 0 0 0 12.561-7.109c16.152-10.818 31.153-23.174 44.914-36.746l8.44 5.664 11.12 7.43c51.365 34.158 101.575 64.836 151.658 70.776a127.34 127.34 0 0 0 16.352 1.034c38.408 0 49.1-47.036 51.238-85.275l.13-2.536c.35-7.556-5.533-13.924-13.079-14.163-67.382-2.111-121.832-19.681-171.436-41.17l-5.595-2.448c31.38-50.714 50.587-108.764 51.032-166.662l.073-.966c.357-4.924-3.569-9.106-8.5-9.106H545.33v-52.055h147.974c8.858 0 12.75-19.747 12.742-35.099-.009-2.398-1.865-4.394-4.267-4.394H545.32v-73.526c0-2.373-1.942-4.267-4.31-4.267M347.707 576.149c48.775 0 95.602 20.015 141.153 46.6l5.152 3.036c-92.478 106.997-220.69 85.627-233.683 23.545a59.723 59.723 0 0 1-1.016-8.778l-.05-2.085.011-.81c.865-35.815 40.48-61.508 88.433-61.508"/>
            </svg>
        }
    }
}
