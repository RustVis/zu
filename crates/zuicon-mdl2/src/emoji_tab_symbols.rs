// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct EmojiTabSymbols {}

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

impl Component for EmojiTabSymbols {
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
                data-icon={ "EmojiTabSymbols" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1920 1152h128v384h-384v-128h192q-45-60-113-94t-143-34q-51 0-98 15t-87 44-70 67-47 87l-120-42q24-67 66-122t97-94 122-61 137-22q91 0 174 35t146 100v-135zM448 0q93 0 174 35t143 96 96 142 35 175q0 93-35 174t-96 143-142 96-175 35q-93 0-174-35t-143-96-96-142T0 448q0-93 35-174t96-143 142-96T448 0zM128 448q0 66 25 124t68 102 102 69 125 25q47 0 92-13t84-40L181 272q-26 39-39 84t-14 92zm587 176q26-39 39-84t14-92q0-66-25-124t-69-101-102-69-124-26q-47 0-92 13t-84 40l443 443zm-203 912h384v128H512v384H384v-384H0v-128h384v-384h128v384zm1088 384q51 0 98-15t86-44 70-67 48-87l120 42q-24 67-66 122t-97 94-122 61-137 22q-91 0-174-35t-146-100v135h-128v-384h384v128h-192q45 60 113 94t143 34zM1230 521q-38-38-58-87t-20-102q0-56 21-105t57-85 84-57 105-21q51 0 97 18t84 52q37-34 83-52t98-18q55 0 104 21t85 57 57 85 21 105q0 53-20 102t-58 87l-370 369-370-369zm190-329q-29 0-54 11t-45 30-30 44-11 55q0 28 10 53t31 45l279 279 279-279q41-41 41-98 0-29-11-54t-30-45-44-30-55-11q-57 0-98 41l-82 82-82-82q-20-20-45-30t-53-11z" />
            </svg>
        }
    }
}
