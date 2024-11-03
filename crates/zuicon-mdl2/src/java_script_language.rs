// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct JavaScriptLanguage {}

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

impl Component for JavaScriptLanguage {
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
                data-icon={ "JavaScriptLanguage" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M385 1534q65 0 104-33t59-84 27-108 6-108V405h145v794q0 82-17 165t-57 149-106 109-163 42q-30 0-60-4t-59-17v-143q26 19 57 26t64 8zm751-832q0 46 17 81t53 64q44 36 96 62t103 52q57 29 115 62t106 78 77 100 30 130q0 96-38 160t-101 102-141 55-161 16q-31 0-73-5t-87-14-84-22-66-30v-171q29 26 70 46t87 35 92 23 84 8q49 0 96-8t85-30 61-59 24-96q0-52-25-91t-68-72-96-59-109-56-110-60-96-72-67-93-26-123q0-88 38-150t100-103 139-59 153-19q30 0 65 1t71 6 69 14 62 23v163q-62-42-134-59t-147-17q-44 0-90 9t-85 31-64 58-25 89z" />
            </svg>
        }
    }
}
