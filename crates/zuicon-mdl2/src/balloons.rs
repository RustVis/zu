// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Balloons {}

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

impl Component for Balloons {
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
                data-icon={ "Balloons" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1728 768q-26 0-45-19t-19-45q0-40-15-75t-41-61-61-41-75-15q-26 0-45-19t-19-45q0-26 19-45t45-19q66 0 124 25t102 68 69 102 25 125q0 26-19 45t-45 19zM576 256q66 0 124 25t102 68 69 102 25 125q0 26-19 45t-45 19q-26 0-45-19t-19-45q0-40-15-75t-41-61-61-41-75-15q-26 0-45-19t-19-45q0-26 19-45t45-19zm960 1257q0 20-1 46t2 49 19 39 44 17q66 0 124 25t102 68 69 102 25 125q0 26-19 45t-45 19q-26 0-45-19t-19-45q0-40-15-75t-41-61-61-41-75-15-75-15-61-41-41-61-15-75v-87q-139-66-247-173t-174-246q-66 94-154 168t-193 123v87q0 40 15 75t41 61 61 41 75 15h100q47 0 90 6t83 28 81 60q46 46 70 103t24 123q0 26-19 45t-45 19q-26 0-45-19t-19-45q0-40-15-75t-41-61-61-41-75-15H832q-66 0-124-25t-102-68-69-102-25-125v-87q-117-55-212-140t-161-190T36 828 0 576q0-119 45-224t124-183T352 46 576 0q77 0 150 20t138 57 119 91 92 121q80-78 182-119t215-42q119 0 224 45t183 124 123 183 46 224q0 130-36 251t-102 227-162 191-212 140zm-960-239q103-47 185-119t141-164 90-196 32-219q0-93-35-174t-96-142-142-96-175-36q-93 0-174 35t-142 96-96 142-36 175q0 113 31 218t90 197 141 163 186 120zm492-319q25 74 65 141t91 124 114 103 134 79q103-47 185-119t141-164 90-196 32-219q0-93-35-174t-96-142-142-96-175-36q-100 0-189 42t-153 120q11 38 16 78t6 80q0 99-21 194t-63 185z" />
            </svg>
        }
    }
}
