// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Eyedropper {}

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

impl Component for Eyedropper {
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
                data-icon={ "Eyedropper" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1758 0q60 0 113 23t92 62 62 92 23 113q0 58-22 111t-63 95l-240 240q33 32 51 72t18 88q0 46-17 86t-50 72l-157 157-160-161-805 805-26 1q-51 2-87 17t-64 36-50 46-48 45-55 34-73 14q-41 0-77-16t-64-43-43-63-16-78q0-43 13-73t35-55 45-47 45-50 37-64 17-88l1-26 805-805-161-160 157-157q32-32 72-49t86-18q47 0 87 18t73 51l240-240q41-41 94-63t112-22zm-441 960l-229-229-769 769q-5 51-19 91t-36 74-51 66-64 67q-21 21-21 51 0 29 21 50t50 21q30 0 51-21 35-34 67-63t66-52 74-36 91-19l769-769zm555-555q48-48 48-114 0-33-13-63t-35-52-52-35-63-13q-66 0-114 48l-331 330-94-94q-28-28-66-28-21 0-39 10t-35 25-32 31-28 30l550 550 68-68q28-28 28-66 0-20-7-36t-22-31l-93-93 330-331z" />
            </svg>
        }
    }
}
