// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct EmptyRecycleBin {}

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

impl Component for EmptyRecycleBin {
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
                data-icon={ "EmptyRecycleBin" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M782 1589q-32 0-48-29-6-11-21-38t-31-56-29-55-13-35q0-16 7-34t18-36 21-34 18-28l-64-51h197l47 194-77-61-55 111h183v152H782zm231-644l-84 165-127-64 60-119q14-30 48-30l221-1q32 0 48 30l54 101 58-44-30 207-218-20 62-46-92-179zm122 719l-124-166 124-152v103h163l-67-123 122-72q4 9 13 25t19 35 16 35 7 27q0 12-6 24l-85 168q-7 14-20 21t-28 8h-134v67zm599-1024h132l-271 1211q-130 97-268 147t-302 50q-162 0-301-50t-268-147L137 441l-6-28q-3-14-3-29 0-58 29-105t78-85 109-65 123-49 121-33 103-20Q856 0 1024 0q51 0 101 2t102 8q-66 20-127 49t-118 70q-48 1-106 5t-120 12-126 21-123 32-111 44-90 58q-18 16-34 37t-16 46q0 15 6 28t16 26 21 23 23 19q42 32 96 56t116 41 126 30 129 19 124 11 111 3h22q11 0 22-1l1 1h632q-74 35-158 59t-173 40-177 22-169 7q-84 0-178-7t-188-25-184-45-164-69l260 1154q100 70 217 105t239 36q121 0 237-35t217-106l254-1136zM2048 0v512h-512V384h292q-76-60-167-91t-188-31q-114 0-218 43t-186 124l-90-90q99-99 226-152t268-53q122 0 237 41t210 119V0h128z" />
            </svg>
        }
    }
}
