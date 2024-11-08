// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Badge {}

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

impl Component for Badge {
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
                data-icon={ "Badge" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1792 641q0 23 12 52t29 60 37 57 34 44q72 86 108 178t36 206q0 108-46 200t-123 167q-38 37-85 71t-98 62-105 50-106 35q-114 28-219 64t-211 89q-16 8-31 8t-31-8q-105-53-210-89t-220-64q-52-13-105-34t-105-50-99-63-85-71q-76-74-122-166T0 1238q0-114 36-205t108-179q14-17 33-44t37-57 30-59 12-53q0-66-27-113t-75-90q-13-11-19-23t-6-30q0-25 18-43L402 83q9-9 21-14t25-5q15 0 25 5t21 15q42 38 87 56t103 18q37 0 78-8t79-25 72-40 57-55q11-14 23-22t31-8q18 0 30 8t24 22q22 31 56 54t72 40 80 25 78 9q57 0 102-18t88-56q11-10 21-15t24-5q28 0 47 19l255 259q18 18 18 43 0 17-6 29t-19 24q-48 42-75 89t-27 114zm-768 1207q81-42 165-72t172-54q24-7 48-12t48-12q52-14 106-37t106-55 97-71 80-87 54-101 20-114q0-73-15-123t-41-91-59-82-67-95q-14-21-27-45t-23-51-17-52-7-53q0-72 25-138t74-119l-167-169q-51 36-110 53t-122 18q-41 0-88-9t-92-25-88-39-72-51q-31 28-72 51t-88 39-92 25-88 9q-62 0-121-17t-111-54L285 384q48 53 73 119t26 138q0 26-6 52t-17 53-24 50-27 46q-34 54-67 94t-58 82-41 92-16 123q0 60 20 114t54 101 80 86 97 71 105 55 107 38q24 7 48 12t48 12q87 23 171 53t166 73z" />
            </svg>
        }
    }
}
