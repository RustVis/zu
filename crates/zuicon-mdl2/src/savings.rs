// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Savings {}

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

impl Component for Savings {
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
                data-icon={ "Savings" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M576 768q26 0 45 19t19 45q0 26-19 45t-45 19q-26 0-45-19t-19-45q0-26 19-45t45-19zm1102-324q84 39 152 99t117 137 75 163 26 181q0 79-18 154t-51 144-80 130-107 113v163q0 40-15 75t-41 61-61 41-75 15h-128q-30 0-58-9t-53-26-42-40-28-53h-278q-10 29-28 52t-42 41-52 26-59 9H704q-40 0-75-15t-61-41-41-61-15-75v-80q-68-18-128-53t-109-84-82-109-51-129q-32-8-58-26t-44-42-29-55-11-62V960q0-34 11-65t33-57 49-43 63-24q74-104 162-194t194-162V57l399 199h123q14-56 45-103t74-81 96-53 111-19q69 0 130 26t107 72 72 107 27 131q0 56-18 108zm-318-316q-43 0-81 16t-66 45-44 66-17 81q0 35 10 65l342 85q30-29 47-68t17-82q0-43-16-81t-45-66-66-44-81-17zm304 1378q57-46 104-99t81-113 52-127 19-143q0-76-22-147t-62-133-96-109-126-80q-17 21-37 38t-44 32l-462-115q-15-31-27-61t-17-65H881L640 264v218q-70 47-126 91t-104 93-92 104-92 126q-17 0-34 1t-32 8-23 19-9 36v128q0 26 19 45t45 19h64q0 80 30 149t82 122 122 83 150 30v192q0 26 19 45t45 19h128q26 0 45-19t19-45v-64h512v40q0 22 4 42t18 33 42 13h128q26 0 45-19t19-45v-222z" />
            </svg>
        }
    }
}
