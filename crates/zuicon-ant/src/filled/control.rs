// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Control {}

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

impl Component for Control {
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
                data-icon={ "control" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32zM404 683v77c0 4.4-3.6 8-8 8h-48c-4.4 0-8-3.6-8-8v-77c-41.7-13.6-72-52.8-72-99s30.3-85.5 72-99V264c0-4.4 3.6-8 8-8h48c4.4 0 8 3.6 8 8v221c41.7 13.6 72 52.8 72 99s-30.3 85.5-72 99zm279.6-143.9c.2 0 .3-.1.4-.1v221c0 4.4-3.6 8-8 8h-48c-4.4 0-8-3.6-8-8V539c.2 0 .3.1.4.1-42-13.4-72.4-52.7-72.4-99.1 0-46.4 30.4-85.7 72.4-99.1-.2 0-.3.1-.4.1v-77c0-4.4 3.6-8 8-8h48c4.4 0 8 3.6 8 8v77c-.2 0-.3-.1-.4-.1 42 13.4 72.4 52.7 72.4 99.1 0 46.4-30.4 85.7-72.4 99.1zM616 440a36 36 0 1 0 72 0 36 36 0 1 0-72 0zM403.4 566.5l-1.5-2.4c0-.1-.1-.1-.1-.2l-.9-1.2c-.1-.1-.2-.2-.2-.3-1-1.3-2-2.5-3.2-3.6l-.2-.2c-.4-.4-.8-.8-1.2-1.1-.8-.8-1.7-1.5-2.6-2.1h-.1l-1.2-.9c-.1-.1-.3-.2-.4-.3-1.2-.8-2.5-1.6-3.9-2.2-.2-.1-.5-.2-.7-.4-.4-.2-.7-.3-1.1-.5-.3-.1-.7-.3-1-.4-.5-.2-1-.4-1.5-.5-.4-.1-.9-.3-1.3-.4l-.9-.3-1.4-.3c-.2-.1-.5-.1-.7-.2-.7-.1-1.4-.3-2.1-.4-.2 0-.4 0-.6-.1-.6-.1-1.1-.1-1.7-.2-.2 0-.4 0-.7-.1-.8 0-1.5-.1-2.3-.1s-1.5 0-2.3.1c-.2 0-.4 0-.7.1-.6 0-1.2.1-1.7.2-.2 0-.4 0-.6.1-.7.1-1.4.2-2.1.4-.2.1-.5.1-.7.2l-1.4.3-.9.3c-.4.1-.9.3-1.3.4-.5.2-1 .4-1.5.5-.3.1-.7.3-1 .4-.4.2-.7.3-1.1.5-.2.1-.5.2-.7.4-1.3.7-2.6 1.4-3.9 2.2-.1.1-.3.2-.4.3l-1.2.9h-.1c-.9.7-1.8 1.4-2.6 2.1-.4.4-.8.7-1.2 1.1l-.2.2a54.8 54.8 0 0 0-3.2 3.6c-.1.1-.2.2-.2.3l-.9 1.2c0 .1-.1.1-.1.2l-1.5 2.4c-.1.2-.2.3-.3.5-2.7 5.1-4.3 10.9-4.3 17s1.6 12 4.3 17c.1.2.2.3.3.5l1.5 2.4c0 .1.1.1.1.2l.9 1.2c.1.1.2.2.2.3 1 1.3 2 2.5 3.2 3.6l.2.2c.4.4.8.8 1.2 1.1.8.8 1.7 1.5 2.6 2.1h.1l1.2.9c.1.1.3.2.4.3 1.2.8 2.5 1.6 3.9 2.2.2.1.5.2.7.4.4.2.7.3 1.1.5.3.1.7.3 1 .4.5.2 1 .4 1.5.5.4.1.9.3 1.3.4l.9.3 1.4.3c.2.1.5.1.7.2.7.1 1.4.3 2.1.4.2 0 .4 0 .6.1.6.1 1.1.1 1.7.2.2 0 .4 0 .7.1.8 0 1.5.1 2.3.1s1.5 0 2.3-.1c.2 0 .4 0 .7-.1.6 0 1.2-.1 1.7-.2.2 0 .4 0 .6-.1.7-.1 1.4-.2 2.1-.4.2-.1.5-.1.7-.2l1.4-.3.9-.3c.4-.1.9-.3 1.3-.4.5-.2 1-.4 1.5-.5.3-.1.7-.3 1-.4.4-.2.7-.3 1.1-.5.2-.1.5-.2.7-.4 1.3-.7 2.6-1.4 3.9-2.2.1-.1.3-.2.4-.3l1.2-.9h.1c.9-.7 1.8-1.4 2.6-2.1.4-.4.8-.7 1.2-1.1l.2-.2c1.1-1.1 2.2-2.4 3.2-3.6.1-.1.2-.2.2-.3l.9-1.2c0-.1.1-.1.1-.2l1.5-2.4c.1-.2.2-.3.3-.5 2.7-5.1 4.3-10.9 4.3-17s-1.6-12-4.3-17c-.1-.2-.2-.4-.3-.5z"/>
            </svg>
        }
    }
}
