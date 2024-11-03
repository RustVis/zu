// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct WechatWork {}

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

impl Component for WechatWork {
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
                data-icon={ "wechat-work" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M845.784 581.585a135.867 135.867 0 0 0-47.042 19.049 114.236 114.236 0 0 1-51.41 31.075c2.613-17.554 11.313-33.577 24.455-45.418a169.303 169.303 0 0 0 23.409-55.017c-.061-27.838 22.44-50.459 50.265-50.534 27.827-.076 50.45 22.423 50.539 50.26.089 27.839-22.39 50.482-50.215 50.585zM753.568 460.83a168.829 168.829 0 0 0-54.808-23.68c-27.836 0-50.402-22.575-50.402-50.423 0-27.848 22.566-50.423 50.402-50.423 27.836 0 50.402 22.575 50.402 50.423a137.497 137.497 0 0 0 18.817 47.211 114.809 114.809 0 0 1 30.764 51.656 76.08 76.08 0 0 1-45.026-24.763h-.186zm-83.033-177.713C655.34 155.789 523.548 56.026 364.09 56.026c-169.874 0-308.088 113.097-308.088 252.19 2.72 78.1 43.941 149.778 110.063 191.385a311.284 311.284 0 0 0 33.602 21.588l-13.664 54.569c4.928 2.316 9.706 4.78 14.746 6.91l68.996-34.512c10.08 2.615 20.683 4.295 31.21 6.088 6.721 1.195 13.442 2.428 20.35 3.25a354.835 354.835 0 0 0 128.805-7.396 248.885 248.885 0 0 0 10.154 55.055 425.638 425.638 0 0 1-96.175 11.242 417.983 417.983 0 0 1-86.392-9.524l-125.186 62.526a27.619 27.619 0 0 1-29.98-3.137 28.019 28.019 0 0 1-9.67-28.611l22.401-90.239C53.176 495.186 2.463 405.506 0 308.216 0 137.973 163.004 0 364.09 0c190.93 0 347.29 124.527 362.521 282.818a244.967 244.967 0 0 0-26.47-2.614c-9.893.374-19.787 1.307-29.607 2.876zM554.237 481.934c16.764-3.362 32.706-9.786 47.042-19.049a114.236 114.236 0 0 1 51.447-31.001 76.466 76.466 0 0 1-24.491 45.344c-11.014 16.807-18.929 35.483-23.409 55.054.041 27.833-22.468 50.435-50.29 50.497-27.821.062-50.43-22.44-50.514-50.273-.082-27.833 22.393-50.469 50.215-50.572m90.798 121.314c16.652 11.168 35.17 19.236 54.659 23.904 20.386 0 38.764 12.286 46.565 31.127 7.801 18.842 3.49 40.53-10.926 54.95-14.414 14.422-36.093 18.736-54.927 10.931-18.834-7.804-31.114-26.19-31.114-46.585a136.736 136.736 0 0 0-18.668-47.285 114.714 114.714 0 0 1-30.54-51.805 76 76 0 0 1 44.951 25.062z" transform="translate(64 148)"/>
            </svg>
        }
    }
}
