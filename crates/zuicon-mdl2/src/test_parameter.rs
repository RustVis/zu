// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct TestParameter {}

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

impl Component for TestParameter {
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
                data-icon={ "TestParameter" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M125 716q0 106 33 191t94 144 147 92 191 32q74 0 147-12t140-45v108q-72 30-150 40t-155 11q-126 0-231-39t-181-111T42 952 0 721q0-129 44-238t122-187 187-123 238-45q115 0 217 34t178 100 121 160 45 216q0 33-4 65t-12 65h-112v186q-74 63-169 63-76 0-119-35t-46-115q-24 72-72 111t-125 39q-55 0-94-22t-65-58-38-83-12-96q0-65 17-130t52-117 89-86 128-33q21 0 43 6t41 18 33 29 22 39l11-81h116l-23 259q-2 29-6 58t-4 59q0 18 2 42t9 45 21 37 40 15q46 0 75-30t47-74 23-91 7-83q0-99-33-176t-92-129-139-79-177-27q-104 0-189 38T252 372t-94 154-33 190zm403 202q34 0 59-14t44-38 31-54 19-62 10-63 3-58q0-29-5-54t-19-43-34-29-54-11q-49 0-81 26t-53 66-28 86-9 85q0 28 6 57t19 52 36 39 56 15zm1435 945q14 28 14 57 0 26-10 49t-27 41-41 28-50 10h-754q-26 0-49-10t-41-27-28-41-10-50q0-29 14-57l299-598v-241h-128V896h640v128h-128v241l299 598zm-242-199l-185-369v-271h-128v271l-185 369h498z" />
            </svg>
        }
    }
}
