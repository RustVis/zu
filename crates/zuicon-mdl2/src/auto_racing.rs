// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AutoRacing {}

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

impl Component for AutoRacing {
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
                data-icon={ "AutoRacing" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 0q132 0 254 34t230 97 194 150 150 195 97 229 35 255q0 132-34 254t-97 230-150 194-195 150-229 97-255 35q-132 0-254-34t-230-97-194-150-150-195-97-229T0 960q0-132 34-254t97-230 150-194 195-150 229-97T960 0zm0 1792q115 0 221-30t198-84 169-130 130-168 84-199 30-221q0-115-30-221t-84-198-130-169-168-130-199-84-221-30q-115 0-221 30t-198 84-169 130-130 168-84 199-30 221q0 115 30 221t84 198 130 169 168 130 199 84 221 30zm172-916q20 40 20 84 0 40-15 75t-41 61-61 41-75 15q-40 0-75-15t-61-41-41-61-15-75q0-40 15-75t41-61 61-41 75-15q44 0 84 20l-1-1 368-368 90 90-368 368-1-1zm-172 148q26 0 45-19t19-45q0-26-19-45t-45-19q-26 0-45 19t-19 45q0 26 19 45t45 19zm640-352q31 68 46 140t16 148q0 140-52 269t-152 229l-91-91q81-81 125-185t44-219q0-124-52-239l116-52zm-404-236q-115-52-239-52-79 0-152 20t-137 59-116 90-89 117-58 137-21 153q0 115 44 220t125 187l-91 91q-100-100-153-228t-53-270q0-97 25-187t71-168 110-142 142-109 168-71 188-25q75 0 147 15t141 47l-52 116z" />
            </svg>
        }
    }
}
