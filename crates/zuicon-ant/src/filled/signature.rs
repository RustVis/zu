// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Signature {}

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

impl Component for Signature {
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
                data-icon={ "signature" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <g><path d="M33.713 640c1.994 0 3.988-.2 5.982-.498l168.19-29.508c1.994-.399 3.888-1.296 5.284-2.792l423.915-423.875a9.927 9.927 0 0 0 0-14.056L470.888 2.891C468.994.997 466.5 0 463.809 0s-5.184.997-7.078 2.891L32.816 426.766c-1.495 1.496-2.393 3.29-2.791 5.284L.514 600.224c-1.894 11.066 1.495 21.932 9.372 29.807 6.58 6.48 14.954 9.969 23.827 9.969M486.826 455.928c27.691-14.812 57.293-20.852 85.545-15.519 32.365 6.11 59.72 26.534 78.96 59.406 29.974 51.211 21.642 102.332-18.484 144.254-17.577 18.364-41.07 35.013-69.996 50.297l-.293.152.848.26c13.153 3.956 27.085 6.1 41.54 6.21l1.174.005c61.068 0 100.981-22.104 125.285-67.876 9.325-17.56 31.119-24.237 48.679-14.913 17.56 9.325 24.237 31.119 14.912 48.68-37.285 70.218-102.092 106.109-188.876 106.109-47.687 0-91.94-15.03-128.188-41.368l-1.056-.774-1.36.473c-46.18 15.996-98.732 29.945-155.37 41.932l-2.239.472c-48.571 10.217-97.257 18.377-139.154 23.957-19.709 2.625-37.813-11.224-40.438-30.932-2.625-19.709 11.224-37.813 30.932-40.438 40.196-5.353 87.126-13.22 133.84-23.045 42.799-9.002 83.011-19.134 119.357-30.342l.234-.074-.436-.693c-16.464-26.452-25.857-55.432-26.142-83.24l-.007-1.303c0-49.907 39.555-104.315 90.733-131.69m72.188 55.231c-10.74-2.027-24.099.699-38.228 8.257-29.546 15.804-52.693 47.643-52.693 68.202 0 18.206 8.889 40.146 24.71 59.736l.238.293 1.223-.514c39.17-16.581 68.483-34.271 85.929-52.186l.64-.663c18.735-19.573 21.386-35.842 8.36-58.1-9.059-15.475-19.03-22.92-30.18-25.025" transform="translate(112 112)"/></g>
            </svg>
        }
    }
}
