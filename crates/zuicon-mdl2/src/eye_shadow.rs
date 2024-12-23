// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct EyeShadow {}

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

impl Component for EyeShadow {
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
                data-icon={ "EyeShadow" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1305 896h743v1024H0V896h771q75-91 154-176t162-168q31-31 86-84t122-114 143-124 148-114 135-83 107-33q41 0 66 25t26 67q0 35-21 82t-56 103-82 116-98 122-104 119-101 108-88 90-65 64zm-412 54q25 11 45 31t32 46q57-47 128-113t149-142 156-158 148-164 126-157 91-141q-64 34-140 90t-158 126-163 148-159 156-142 149-113 129zm-101 107q-24 0-57 7t-69 18-74 27-72 33-62 36-45 37q-16 17-22 36t-7 42q0 44 21 87t56 78 78 56 88 22q23 0 42-6t36-23q18-17 36-44t36-62 33-72 27-74 19-69 7-58q0-41-15-56t-56-15zm1128 735v-768h-752q-45 40-89 78t-92 77q-5 45-23 104t-45 118-59 111-65 85q-35 35-78 51t-92 16q-71 0-137-31t-118-83-83-118-31-138q0-54 17-94t48-72 69-56 83-48H128v768h1792zm-768-384q0-53 20-99t55-82 81-55 100-20q53 0 99 20t82 55 55 81 20 100q0 53-20 99t-55 82-81 55-100 20q-53 0-99-20t-82-55-55-81-20-100zm256-128q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10z" />
            </svg>
        }
    }
}
