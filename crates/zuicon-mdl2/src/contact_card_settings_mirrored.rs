// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ContactCardSettingsMirrored {}

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

impl Component for ContactCardSettingsMirrored {
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
                data-icon={ "ContactCardSettingsMirrored" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M256 896h640V768H256v128zM0 256v987q28-35 59-66t69-56V384h1792v1280h-903q-3 33-10 65t-18 63h1059V256H0zm1792 1152q0-52-14-101t-40-93-63-80-83-61q34-35 53-81t19-96q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 50 19 96t53 81q-46 25-83 61t-63 79-40 93-14 102h128q0-53 20-99t55-82 81-55 100-20q53 0 99 20t82 55 55 81 20 100h128zm-384-640q27 0 50 10t40 27 28 41 10 50q0 27-10 50t-27 40-41 28-50 10q-27 0-50-10t-40-27-28-41-10-50q0-27 10-50t27-40 41-28 50-10zM10 1488l124 51q-6 30-6 61t6 61l-124 51 49 119 124-52q35 51 86 86l-52 124 119 49 51-124q30 6 61 6t61-6l51 124 119-49-52-124q51-35 86-86l124 52 49-119-124-51q6-30 6-61t-6-61l124-51-49-119-124 52q-35-51-86-86l52-124-119-49-51 124q-30-6-61-6t-61 6l-51-124-119 49 52 124q-51 35-86 86l-124-52-49 119zm438 304q-40 0-75-15t-61-41-41-61-15-75q0-40 15-75t41-61 61-41 75-15q40 0 75 15t61 41 41 61 15 75q0 40-15 75t-41 61-61 41-75 15z" />
            </svg>
        }
    }
}
