// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct VerifiedBrand {}

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

impl Component for VerifiedBrand {
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
                data-icon={ "VerifiedBrand" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1845 1024l124 155q18 23 28 50t10 57q0 30-9 57t-26 49-41 38-52 24l-191 53q2 51 5 103t4 104q0 36-13 67t-37 54-55 37-68 14q-31 0-61-11l-185-70-109 165q-24 36-62 57t-83 21q-44 0-82-21t-63-57l-109-165-185 70q-30 11-61 11-36 0-67-13t-55-37-37-55-14-67q0-52 3-104t6-103l-191-53q-29-8-52-24t-40-38-26-49-10-57q0-29 10-56t28-51l124-155L79 869q-38-47-38-107 0-30 9-57t26-49 40-38 53-24l191-53q-2-51-5-103t-4-104q0-36 13-67t37-54 55-37 68-14q31 0 61 11l185 70L879 78q24-36 62-57t83-21q44 0 82 21t63 57l109 165 185-70q30-11 61-11 36 0 67 13t55 37 37 55 14 67q0 52-3 104t-6 103l191 53q28 8 52 24t40 38 26 49 10 57q0 60-38 107l-124 155zm34 262q0-14-10-27l-188-235 188-235q10-13 10-27 0-34-34-44l-191-53-98-27q3-76 8-152t5-152q0-17-14-30t-31-14q-6 0-27 6t-49 17-62 24-64 25-56 22-39 15l-56-85-109-166q-13-20-38-20t-38 20L877 314l-56 85q-13-5-38-15t-57-22-64-25-61-23-50-17-27-7q-17 0-31 13t-14 31q0 76 5 152t8 152l-98 27-191 53q-34 10-34 44 0 14 10 27l188 235-188 235q-10 13-10 27 0 34 34 44l191 53 98 27q-3 76-8 152t-5 152q0 17 14 30t31 14q6 0 27-6t49-17 62-24 64-25 56-22 39-15l56 85 109 166q13 20 38 20t38-20l109-166 56-85q13 5 38 15t57 22 64 25 61 23 50 17 27 7q17 0 31-13t14-31q0-76-5-152t-8-152l98-27 191-53q34-10 34-44zm-983-97l467-466 90 90-557 558-301-302 90-90 211 210z" />
            </svg>
        }
    }
}
