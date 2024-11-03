// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ReadOutLoud {}

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

impl Component for ReadOutLoud {
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
                data-icon={ "ReadOutLoud" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M928 1152l384-384h96v1280h-96l-384-384H640v-512h288zm352 683V981l-299 299H768v256h213l299 299zm325-880q91 91 139 208t48 245q0 128-48 245t-139 208l-91-91q72-72 111-165t39-197q0-103-39-196t-111-166l91-91zm443 453q0 180-68 343t-194 291l-91-91q109-109 167-249t58-294q0-154-58-294t-167-249l91-91q126 128 194 291t68 343zM614 128H128v1536h384v128H0V0h614q108 0 197 42t149 135q58-93 148-135t198-42h614v685q-26-35-61-67t-67-63V128h-486q-88 0-155 37t-105 119q-3 6-6 16t-7 21-6 21-3 18v470L896 958V360q0-7-2-17t-6-22-7-21-7-16q-37-82-104-119t-156-37z" />
            </svg>
        }
    }
}
