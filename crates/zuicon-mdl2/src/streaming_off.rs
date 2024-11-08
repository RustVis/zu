// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct StreamingOff {}

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

impl Component for StreamingOff {
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
                data-icon={ "StreamingOff" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1269 1024q0 51-19 95t-53 78-77 52-96 20q-51 0-95-19t-78-53-52-77-20-96q0-51 19-95t53-78 77-52 96-20q51 0 95 19t78 53 52 77 20 96zm-245 139q29 0 54-11t44-29 30-44 11-55q0-29-11-54t-29-44-44-30-55-11q-29 0-54 11t-44 29-30 44-11 55q0 29 11 54t29 44 44 30 55 11zM383 383q-64 64-113 138t-84 155-51 169-18 179q0 91 17 178t52 169 83 156 114 138l-75 75q-144-144-220-328t-77-388q0-204 76-388t221-328l75 75zm271 271q-75 75-114 170t-39 200q0 105 39 200t114 170l-75 75q-89-90-136-204t-48-241q0-126 47-240t137-205l75 75zm946 338q-27 0-54 3-5-97-44-184t-108-157l75-75q84 84 130 191t52 225q-26-3-51-3zm328 98q2-17 2-33t0-32q0-90-17-178t-51-169-83-156-114-139l75-75q71 71 126 154t92 174 57 189 20 199q0 36-2 72t-8 72q-22-22-46-41t-51-37zm-328 62q93 0 174 35t143 96 96 142 35 175q0 93-35 174t-96 143-142 96-175 35q-93 0-174-35t-143-96-96-142-35-175q0-93 35-174t96-143 142-96 175-35zm-320 448q0 66 25 124t68 102 102 69 125 25q47 0 92-13t84-40l-443-443q-26 39-39 84t-14 92zm587 176q26-39 39-84t14-92q0-66-25-124t-69-101-102-69-124-26q-47 0-92 13t-84 40l443 443z" />
            </svg>
        }
    }
}
