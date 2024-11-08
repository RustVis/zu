// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Webcam2off {}

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

impl Component for Webcam2off {
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
                data-icon={ "Webcam2Off" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 128q-119 0-231 35T584 267l-91-91Q609 90 744 45t280-45q123 0 237 32t214 90 182 141 140 181 91 214 32 238q0 144-45 280t-131 251l-94-93q49-70 82-147t49-163h-330q-6 20-14 39t-18 39l-96-96q21-52 21-110 0-66-25-124t-68-102-102-69-125-25q-58 0-110 21l-96-96q48-26 100-39t106-14q93 0 174 35t142 96 96 142 36 175h320q0-106-27-204t-78-183-120-156-155-120-184-77-204-28zm513 1500q-119 84-257 126v166h384v128H384v-128h384v-166q-142-42-260-126t-202-197-131-249-47-286q0-139 42-270t122-244L19 109l90-90 1920 1920-90 90-402-401zM717 808q-6 22-9 43t-4 45q0 66 25 124t68 102 102 69 125 25q23 0 44-3t44-10L717 808zM383 474q-63 94-95 201t-32 221h320q0-98 43-187L383 474zm769 1446v-138q-32 5-64 7t-64 3q-32 0-64-2t-64-8v138h256zm-128-256q113 0 219-33t201-96l-232-233q-89 42-188 42-74 0-142-23t-124-66-98-101-63-130H267q24 138 92 255t170 203 228 134 267 48z" />
            </svg>
        }
    }
}
