// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct MicOff2 {}

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

impl Component for MicOff2 {
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
                data-icon={ "MicOff2" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M640 324L512 196v-4q0-40 15-75t41-61 61-41 75-15h512q40 0 75 15t61 41 41 61 15 75v900l-128-128V192q0-27-18-45t-46-19H704q-27 0-45 18t-19 46v132zm1024 700v323l-128-127v-196h128zm365 915l-90 90-377-377q-59 67-139 103t-169 37h-230v128h256v128H640v-128h256v-128H666q-85 0-159-32t-130-88-88-130-33-160v-358h128v358q0 58 22 109t61 90 89 60 110 23h588q63 0 119-27t98-75l-102-103q-27 36-67 56t-86 21H704q-40 0-75-15t-61-41-41-61-15-75V603L19 109l90-90 1920 1920zm-813-531q20 0 36-11t24-30L640 731v613q0 27 18 45t46 19h512z" />
            </svg>
        }
    }
}
