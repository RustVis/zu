// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct GitFork {}

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

impl Component for GitFork {
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
                data-icon={ "GitFork" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2560 1024q0 47-18 89t-51 76l-790 790q-33 33-75 51t-91 18q-51 0-99-23l944-948q22-22 22-54t-22-54L1438 22q46-21 99-21 46 0 89 17t75 49l790 790q33 33 51 76t18 91zm-510 0q0 46-18 89t-50 76l-791 790q-33 33-75 51t-91 18q-47 0-90-18t-76-51L69 1189q-33-33-51-75t-18-90q0-47 18-90t51-77L859 67q32-32 75-49t90-18q47 0 90 17t77 50l791 790q33 33 50 76t18 91zm-1025 866q32 0 54-22l791-791q22-22 22-54t-22-54l-791-791q-22-22-54-22t-54 22L789 360l175 175q31-13 61-13 33 0 61 12t50 34 34 50 13 62q0 30-13 62l184 184q30-15 65-15 33 0 61 12t50 33 34 50 13 62q0 33-12 61t-34 50-50 34-62 13q-33 0-61-12t-50-34-33-50-13-62q0-28 11-56l-186-187-3 1v445q45 18 72 58t27 88q0 33-12 61t-34 50-50 34-62 13q-33 0-61-12t-50-34-34-50-13-62q0-48 27-88t72-58V826q-45-18-72-58t-27-88q0-30 13-61L705 444 180 969q-11 11-17 25t-6 29q0 31 23 54l791 791q22 22 54 22z" />
            </svg>
        }
    }
}
