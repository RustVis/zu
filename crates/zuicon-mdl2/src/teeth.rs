// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Teeth {}

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

impl Component for Teeth {
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
                data-icon={ "Teeth" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 512q70 0 132-20t124-44 123-44 133-20v128q-44 0-84 9t-78 24-78 31-81 31-89 23-102 10q-55 0-102-9t-89-24-80-31-78-31-79-23-84-10V384q69 0 132 20t124 44 124 44 132 20zm1024 67q0 177-26 360t-76 363-126 349-173 319q-24 36-61 57t-81 21q-55 0-99-31t-61-84l-178-532q-35 4-71 5t-72 2q-36 0-72-1t-71-6l-178 532q-17 52-61 83t-99 32q-44 0-81-21t-61-57q-98-149-173-318t-125-349-77-363T0 579q0-112 34-216t99-185T293 49 510 0q71 0 133 20t124 44 123 44 134 20q71 0 133-20t124-44 123-44 134-20q122 0 217 48t160 129 99 185 34 217zm-128 2q0-93-25-175t-75-144-126-98-175-36q-58 0-111 20t-112 44-124 44-148 20q-83 0-148-20t-124-44-114-44-122-20q-97 0-170 38T225 267t-73 144-24 170q0 98 11 211t34 234 55 241 75 236 95 216 112 182q15 19 38 19 12 0 31-36t40-94 45-126 43-133 37-118 24-77q8-26 30-45t52-20q44 0 87 4t87 5q44 0 87-4t87-5q29 0 51 19t31 46q8 24 24 77t36 117 44 134 44 126 41 93 31 37q23 0 38-19 60-80 112-182t94-216 75-235 55-241 34-234 12-212z" />
            </svg>
        }
    }
}
