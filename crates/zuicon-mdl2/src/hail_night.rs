// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct HailNight {}

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

impl Component for HailNight {
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
                data-icon={ "HailNight" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1280 1296q23 0 43 9t36 24 24 35 9 44q0 23-9 43t-24 36-35 24-44 9q-23 0-43-9t-36-24-24-35-9-44q0-23 9-43t24-36 35-24 44-9zm0 384q23 0 43 9t36 24 24 35 9 44q0 23-9 43t-24 36-35 24-44 9q-23 0-43-9t-36-24-24-35-9-44q0-23 9-43t24-36 35-24 44-9zm-384-256q23 0 43 9t36 24 24 35 9 44q0 23-9 43t-24 36-35 24-44 9q-23 0-43-9t-36-24-24-35-9-44q0-23 9-43t24-36 35-24 44-9zm0 384q23 0 43 9t36 24 24 35 9 44q0 23-9 43t-24 36-35 24-44 9q-23 0-43-9t-36-24-24-35-9-44q0-23 9-43t24-36 35-24 44-9zm865-772q63 17 115 52t91 85 60 109 21 126q0 80-30 149t-82 122-123 83-149 30h-128v-128h128q53 0 99-20t82-55 55-81 20-100q0-55-20-102t-57-81-84-53-102-20q-12-82-51-152t-98-122-134-81-158-29q-76 0-146 25t-127 69-98 106-61 135q-44-38-97-58t-111-21q-66 0-124 25t-102 68-69 102-25 125q0 66 25 124t68 102 102 69 125 25h64v128h-64q-93 0-174-35t-142-96-96-142-36-175q0-77 25-149t74-132q-81-54-136-134T14 754q58 14 114 14 106 0 199-40t163-109 110-163 40-200q0-29-4-57t-10-57q87 20 160 67t126 113 82 147 30 171v17q0 9-1 17 92-34 193-34 93 0 178 29t158 81 126 125 83 161zM324 974q56-38 120-58t132-20q74 0 147 25 61-102 157-172 8-26 12-53t4-56q0-85-35-161T761 348q-15 105-62 197T581 708 417 827t-197 62q22 26 48 47t56 38z" />
            </svg>
        }
    }
}
