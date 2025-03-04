// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Commitments {}

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

impl Component for Commitments {
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
                data-icon={ "Commitments" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1344 0q26 0 45 19l640 640q19 19 19 45t-19 45l-243 243q25 25 49 48t42 47 30 54 12 67q0 40-15 76t-41 63-61 43-75 17q-2 41-17 75t-40 60-60 41-75 16q-1 40-17 75t-43 61-63 42-76 15q-32 0-65-11-11 31-29 56t-44 44-55 29-63 10q-26 0-46-4t-37-14-34-22-35-29q-35 32-68 50t-84 19q-40 0-76-15t-63-41-43-61-17-76q-39-1-74-16t-61-41-41-61-16-74q-40-1-74-16t-60-41-41-61-17-74q-41-1-76-17t-61-43-41-62-15-76q0-41 15-77t44-65l106-107L19 749Q0 730 0 704t19-45L659 19q19-19 45-19t45 19l132 132q13-4 50-17t87-29 103-35 102-34 81-26 40-10zM312 960q-29 0-50 21l-113 112q-21 21-21 51t20 50 51 21q31 0 51-20l113-112q21-21 21-51t-21-51-51-21zm192 192q-14 0-27 5t-24 16l-112 113q-21 21-21 51 0 29 21 49t51 21q14 0 27-5t24-15l112-113q21-21 21-50 0-30-21-51t-51-21zm192 192q-29 0-50 21l-113 112q-21 21-21 51t20 50 51 21q31 0 51-20l113-112q21-21 21-51t-21-51-51-21zm8 377q0 29 21 49t51 21q14 0 27-5t24-15l112-113q21-21 21-50 0-30-21-51t-51-21q-14 0-27 5t-24 16l-112 113q-21 21-21 51zm1016-441q29 0 50-21t22-50q0-30-21-51l-632-633q-3-3-10-5t-14-4-15-3-12-1q-26 0-49 10t-41 27-28 41-10 50q0 53-20 99t-55 82-81 55-100 20q-40 0-75-15t-61-41-41-61-15-75q0-24 5-53t16-52l187-375q9-16 27-26l-43-43-550 549 135 134q45-5 85 6t70 37 48 63 19 83q40 2 75 16t60 40 41 60 16 76q40 2 75 16t60 40 41 60 16 76q41 2 76 17t61 41 41 62 15 77q0 45-18 84t-52 68q15 14 27 23t35 9q29 0 50-21t22-50q0-30-21-51l90-91 65 64q23 21 50 21 29 0 50-21t22-50q0-30-21-51l-113-113 91-90 112 112q21 21 51 21t51-21 21-51q0-30-21-51l-113-112 91-91 113 113q21 21 50 21zm173-576l-566-567-504 168-174 348q-5 9-7 25t-2 26q0 26 19 45t45 19q27 0 50-10t40-27 28-41 10-50q0-53 20-99t55-82 81-55 100-20q16 0 34 3t38 8 35 14 29 21l472 472 197-198z" />
            </svg>
        }
    }
}
