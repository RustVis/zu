// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct WorldClock {}

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

impl Component for WorldClock {
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
                data-icon={ "WorldClock" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M896 768H512V256h128v384h256v128zm1152 640q0 87-22 168t-64 152-100 130-128 101-152 66-168 23q-134 0-251-49t-205-136-139-204-51-251q0-132 50-248t138-204 203-137 249-51q132 0 248 50t204 138 137 203 51 249zm-640 512q21 0 37-15t29-40 21-53 15-58 9-53 5-37h-230q1 13 5 37t10 52 15 58 21 54 27 39 36 16zm125-384q3-64 3-128 0-63-3-128h-250q-3 65-3 128 0 64 3 128h250zm-637-128q0 32 4 64t12 64h243q-6-128 0-256H912q-8 32-12 64t-4 64zm512-512q-19 0-34 15t-27 40-21 54-15 58-11 53-5 36h225q-1-11-5-34t-11-52-16-59-21-54-27-41-32-16zm253 384q3 64 3 128t-2 128h242q8-32 12-64t4-64q0-32-4-64t-12-64h-243zm190-128q-43-75-108-131t-145-89q20 53 32 108t20 112h201zm-637-218q-78 32-142 88t-107 130h200q7-56 18-110t31-108zm-249 730q42 73 105 129t142 88q-20-52-30-107t-17-110H965zm643 215q77-32 139-87t104-128h-198q-5 55-15 109t-30 106zM640 0q88 0 170 23t153 64 129 100 100 130 65 153 23 170h-128q0-106-40-199t-110-162-163-110-199-41q-106 0-199 40T279 278 169 441t-41 199q0 106 40 199t110 162 163 110 199 41v128q-88 0-170-23t-153-64-129-100T88 963 23 810 0 640q0-132 50-248t138-204T391 51 640 0z" />
            </svg>
        }
    }
}
