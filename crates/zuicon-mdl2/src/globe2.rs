// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Globe2 {}

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

impl Component for Globe2 {
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
                data-icon={ "Globe2" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1897 1572q0 26-19 45t-45 19q-15 0-31-10t-31-23-28-29-23-25q-106 94-232 151t-267 77v143h187v128H896v-128h197v-134q-134 0-257-34t-230-96-196-149-150-194-97-230-35-257q0-177 61-337t176-295q-9-8-18-17t-18-19-14-21-6-25q0-26 19-45t45-19q27 0 46 19l222 221q95-78 209-119t238-41q97 0 187 25t168 71 143 110 110 142 71 169 25 187q0 123-41 237t-119 210l246 247q19 19 19 46zM512 832q0 81 23 161l129-130q-37-55-66-113t-49-122q-37 100-37 204zm244-431q-51 0-75 24t-24 76q0 33 9 69t23 71 32 69 36 61l270-270q-28-18-61-36t-69-32-71-23-70-9zm663 492l-270 270q28 18 61 35t68 32 72 24 69 9q52 0 76-24t24-76q0-33-9-69t-23-71-32-69-36-61zm-376 195l301-301q-95-116-211-211L832 877q95 116 211 211zM741 967l-152 153q38 66 91 119t120 92l153-152q-117-95-212-212zm316 289l-130 129q80 23 161 23 104 0 204-37-63-20-121-49t-114-66zm570-220q37-100 37-204 0-81-23-161l-129 130q37 55 66 113t49 122zm-40-492q-38-66-91-119t-120-92l-153 152q117 95 212 212l152-153zm-338-265q-80-23-161-23-104 0-204 37 63 20 121 49t114 66l130-129zm-161 1379q149 0 288-51t253-149l-87-87q-97 80-212 122t-242 43q-97 0-187-25t-168-71-143-110-110-142-71-169-25-187q0-127 42-242t123-212l-93-93q-97 114-148 253t-52 289q0 115 29 221t84 198 130 168 168 130 199 84 222 30z" />
            </svg>
        }
    }
}
