// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct FormatPainter {}

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

impl Component for FormatPainter {
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
                data-icon={ "FormatPainter" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2037 256q0 48-18 93t-53 80l-339 339 120 120q27 27 41 62t15 74q0 38-14 73t-42 63q-26 26-52 54t-55 51l-752 616L11 857l103-23q83-19 147-37t116-41 97-49 87-65 88-84 99-111q33-38 68-74t72-72q27-27 62-41t74-15q38 0 73 14t63 42l120 120 339-339q35-35 80-53t93-18q51 0 95 19t78 52 53 78 19 96zM244 933l87 102 987 329 187-153-670-669q-49 57-91 102t-84 82-83 66-92 54-108 46-133 41zm249 291l411 479 297-243-708-236zm1416-968q0-24-9-45t-25-37-37-25-46-10q-48 0-82 34l-430 430-211-211q-19-19-45-19-16 0-30 8t-27 20-24 25-21 22l678 678q9-10 22-21t25-24 20-27 8-30q0-26-19-45l-211-211 430-430q34-34 34-82z" />
            </svg>
        }
    }
}
