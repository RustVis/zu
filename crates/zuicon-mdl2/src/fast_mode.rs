// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct FastMode {}

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

impl Component for FastMode {
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
                data-icon={ "FastMode" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1428 385q53-4 110 6t111 33 104 56 87 77 58 96 22 111q0 63-26 111t-70 82-99 49-113 17q-43 0-81-7 2 18 3 36t2 36q0 51-8 97t-23 95H415q-12-49-21-95t-10-97q0-16 1-32t3-32q-54 0-101-19t-82-54-56-82-21-101q0-53 20-99t55-82 81-55 100-20q69 0 126 33t94 93q85-69 176-97t200-29q-42-51-63-111t-21-126q0-42 11-78t29-69 40-64 46-63l406 384zM256 768q0 27 10 50t27 40 41 28 50 10q20 0 38-6 31-85 85-154-11-41-45-68t-78-28q-27 0-50 10t-40 27-28 41-10 50zm1147 384q3-16 4-32t1-32q0-30-5-59t-13-58q-42-21-80-50t-70-65-53-78-29-90q-46-23-96-35t-102-13q-91 0-172 36t-142 97-98 143-36 172q0 16 1 32t4 32h240q11-38 11-75 0-57-24-108t-68-89l82-98q66 55 101 130t36 163q0 20-1 39t-6 38h515zm210-257q30 0 62-7t58-23 42-41 17-61q0-55-33-101t-82-79-107-52-107-19q-20 0-39 3t-39 6l-342-323q-9 21-14 37t-5 40q0 38 11 73t31 65 48 53 63 39q32 14 67 18t70 9q-9 25-20 52t-11 55q0 59 32 106t81 80 107 52 110 18z" />
            </svg>
        }
    }
}
