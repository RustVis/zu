// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Timer {}

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

impl Component for Timer {
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
                data-icon={ "Timer" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1639 500q135 136 208 297t73 355q0 124-32 238t-90 214-140 181-181 140-214 91-239 32q-124 0-238-32t-214-90-181-140-140-181-91-214-32-239q0-111 26-216t75-198 118-172 154-141 185-103 210-57V128H640V0h640v128h-256v128q139 0 270 41t245 122l208-208 90 90-198 199zm-615 1420q159 0 298-60t244-165 165-244 61-299q0-159-60-298t-165-244-244-165-299-61q-159 0-298 60T482 609 317 853t-61 299q0 159 60 298t165 244 244 165 299 61zm640-896v128q0 133-50 249t-137 204-203 137-250 50q-133 0-249-50t-204-136-137-203-50-250q0-88 23-170t64-153 100-129 130-101 152-65 171-23h128v512h512zm-640 640q106 0 199-40t163-109 110-163 40-200h-512V640q-107 0-200 40T662 790 552 954t-40 200q0 106 40 199t110 162 163 109 199 40z" />
            </svg>
        }
    }
}
