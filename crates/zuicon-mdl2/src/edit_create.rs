// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct EditCreate {}

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

impl Component for EditCreate {
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
                data-icon={ "EditCreate" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M226 701q-48-48-73-109t-25-128q0-71 26-131t71-105 107-70 131-26q66 0 128 23t110 71l353 353-91 91-195-195-293 293 195 195-91 91-353-353zm238-443q-43 0-81 15t-66 44-44 65-17 82q0 38 10 66t29 53 41 47 48 47l293-293q-26-25-47-48t-46-40-52-28-68-10zm1584 1790l-633-158-293-293 91-91 217 218q16-52 44-98t67-85 84-66 99-45l-218-217 91-91 293 293 158 633zm-176-176l-82-329q-47 10-87 32t-73 55-55 73-32 87l329 82zM1728 192q53 0 99 20t82 55 55 81 20 100q0 51-19 98t-56 83L763 1775q-9 59-37 108t-70 87-95 57-113 21H0v-128q11 0 23-3t22-9q25-13 41-33t25-44 13-50 4-53q0-59 20-112t58-96 86-70 109-37L1547 267q36-36 83-55t98-20zM448 1920q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75q0 104-64 192h256zm518-529q-32-63-75-106t-106-75l-221 221q62 24 109 71t72 110l221-221zm852-853q37-37 37-90 0-26-10-49t-27-40-41-28-49-10q-53 0-90 37l-759 758q61 36 103 78t78 103l758-759z" />
            </svg>
        }
    }
}
