// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PromotedDatabase {}

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

impl Component for PromotedDatabase {
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
                data-icon={ "PromotedDatabase" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M128 637v963q0 20 13 35t27 28q46 38 114 63t142 39 147 21 133 6q81 0 161-8t159-28v130q-79 18-159 26t-161 8q-61 0-121-4t-121-15q-38-6-87-17t-100-29-101-40-88-54-62-72-24-89V448q0-69 39-119t103-86 142-58 157-35 148-17 115-5q45 0 115 4t148 17 157 35 142 58 102 86 40 120v960h-128V637q-58 37-130 62t-148 40-154 22-144 7q-68 0-144-6t-153-22-149-41-130-62zm0-189q0 20 13 35t27 28q46 38 114 63t142 39 147 21 133 6q58 0 131-6t148-20 142-40 114-63q14-11 27-27t14-36q0-20-13-36t-28-27q-47-38-114-63t-141-39-148-21-132-6q-59 0-132 6t-148 20-142 40-114 63q-14 12-27 27t-13 36zm1811 915l90 90-557 558-269-270 90-90 179 178 467-466z" />
            </svg>
        }
    }
}
