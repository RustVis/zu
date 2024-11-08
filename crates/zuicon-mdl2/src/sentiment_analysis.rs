// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SentimentAnalysis {}

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

impl Component for SentimentAnalysis {
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
                data-icon={ "SentimentAnalysis" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1344 640q97 0 187 25t168 71 143 110 110 142 71 169 25 187q0 97-25 187t-71 168-110 143-142 110-169 71-187 25q-97 0-187-25t-168-71-143-110-110-142-71-169-25-187q0-32 3-64-133 0-250-50t-204-136T51 891 0 640q0-88 23-170t64-153 100-129T317 88t153-65T640 0q89 0 171 23t153 65 129 100 100 130 64 154 23 171q31-3 64-3zm-704 512q7 0 14-1t14-2q18-63 47-120t69-110q-35-11-71-17t-73-6q-61 0-119 15t-113 43l-58-114q68-34 141-53t149-19q62 0 121 13t117 36q59-52 127-90t144-59l2-14q1-7 1-14 0-106-40-199t-110-162-163-110-199-41q-106 0-199 40T279 278 169 441t-41 199q0 106 40 199t110 162 163 110 199 41zm704 768q119 0 224-45t183-124 123-183 46-224q0-119-45-224t-124-183-183-123-224-46q-119 0-224 45T937 937t-123 183-46 224q0 119 45 224t124 183 183 123 224 46zm256-608q-40 0-68-28t-28-68q0-40 28-68t68-28q40 0 68 28t28 68q0 40-28 68t-68 28zm-512 0q-40 0-68-28t-28-68q0-40 28-68t68-28q40 0 68 28t28 68q0 40-28 68t-68 28zm256 352q43 0 84-11t77-33 66-52 51-68l111 63q-30 53-72 95t-92 72-107 46-118 16q-61 0-118-16t-107-45-92-72-72-96l111-63q21 38 51 68t66 51 76 33 85 12zM448 528q-33 0-56-23t-24-57q0-33 23-56t57-24q33 0 56 23t24 57q0 33-23 56t-57 24zm384 0q-33 0-56-23t-24-57q0-33 23-56t57-24q33 0 56 23t24 57q0 33-23 56t-57 24z" />
            </svg>
        }
    }
}
