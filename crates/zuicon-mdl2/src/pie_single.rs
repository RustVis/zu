// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PieSingle {}

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

impl Component for PieSingle {
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
                data-icon={ "PieSingle" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1152 0q124 0 238 32t214 90 181 140 140 181 91 214 32 239h-896V0zm128 768h630q-21-121-76-226t-139-189-188-138-227-77v630zm-256 256h896q0 59-3 114t-11 109-23 107-38 108q-57 134-148 242t-206 184-251 118-280 42q-133 0-255-34t-230-96-194-150-150-195-97-229-34-256q0-133 34-255t96-230 150-194 195-150 229-97 256-34h64v896zm-128 128V258q-108 8-207 42t-184 91-155 131-119 165-76 191-27 210q0 115 30 221t84 198 130 169 168 130 199 84 221 30q108 0 209-27t191-76 165-119 132-155 90-184 43-207H896z" />
            </svg>
        }
    }
}
