// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct WebEnvironment {}

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

impl Component for WebEnvironment {
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
                data-icon={ "WebEnvironment" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1920 1280l128 640H0l128-640h232q-51-88-77-185t-27-199q0-106 27-204t78-183 120-156 155-120 184-77 204-28q106 0 204 27t183 78 156 120 120 155 77 184 28 204q0 102-26 199t-78 185h232zm-256-384q0-32-3-64t-10-64h-249q6 65 6 128 0 64-6 128h249q6-32 9-64t4-64zm-890 128h500q6-64 6-128 0-63-6-128H774q-6 65-6 128 0 64 6 128zm482 128H792q4 23 14 60t25 81 35 86 44 78 53 57 61 22q32 0 60-22t53-57 44-78 35-86 25-80 15-61zm-477 335q-46-76-74-162t-43-173H438q24 53 60 103t80 95 96 79 105 58zm607-335q-16 86-43 172t-73 163q54-24 104-59t95-78 81-94 60-104h-224zm224-512q-51-113-138-200t-203-135q46 76 73 162t44 173h224zm-586-384q-32 0-60 22t-53 57-44 78-35 86-26 80-14 61h464q-5-23-14-60t-25-81-35-86-44-78-53-57-61-22zm-245 49q-115 48-202 135T438 640h224q16-87 43-173t74-162zM397 768q-6 32-9 64t-4 64q0 32 3 64t10 64h249q-6-64-6-128 0-63 6-128H397zM156 1792h1736l-77-384h-220q-54 61-119 108t-139 81-152 50-161 17q-82 0-161-17t-152-50-138-80-120-109H233l-77 384z" />
            </svg>
        }
    }
}
