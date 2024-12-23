// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct HomeGroup {}

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

impl Component for HomeGroup {
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
                data-icon={ "HomeGroup" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M320 640q66 0 124 25t101 69 69 102 26 124q0 66-25 124t-69 102-102 69-124 25q-66 0-124-25t-102-68-69-102T0 960q0-66 25-124t68-101 102-69 125-26zm0 512q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75q0 40 15 75t41 61 61 41 75 15zm1532 78q46 30 82 71t62 89 38 101 14 109q0 93-35 174t-96 143-142 96-175 35q-93 0-174-35t-143-96-96-142-35-175q0-50 10-94-88-29-160-83t-125-126-80-158-29-179q0-119 45-224t124-183 183-123 224-46q18 0 36 1t36 3q-8-35-8-68 0-66 25-124t68-101 102-69 125-26q66 0 124 25t101 69 69 102 26 124q0 53-17 102t-48 90-74 71-94 45q52 73 78 157t27 175q0 71-17 139t-51 131zm-316-910q0 62 37 111 35 15 68 35t63 44q6 1 12 1t12 1q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75zM896 960q0 72 22 139t64 124 98 99 126 64q62-109 165-171t229-63q71 0 137 21 27-49 41-103t14-110q0-95-38-181t-108-150l-4-1q-35-29-69-50t-77-39l-1-1q-73-26-151-26-93 0-174 35t-142 96-96 142-36 175zm704 960q66 0 124-25t101-68 69-102 26-125q0-70-26-128t-71-101-105-67-128-24q-64 0-120 26t-99 71-66 102-25 121q0 66 25 124t68 102 102 69 125 25z" />
            </svg>
        }
    }
}
