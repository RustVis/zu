// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct LinuxLogo32 {}

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

impl Component for LinuxLogo32 {
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
                data-icon={ "LinuxLogo32" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1700 1428q32 14 48 38t23 58q4 20 9 37t12 32 16 29 24 31q13 14 23 36t10 42q0 23-12 39t-30 28q-23 16-48 28t-50 25q-41 20-71 42t-60 56q-16 18-36 35t-44 32-49 22-50 9q-58 0-96-20t-66-73q-8-15-16-19t-26-7q-51-4-101-8t-102-4q-42 0-86 7t-87 17q-9 2-18 15t-25 29-43 28-71 13q-33 0-71-7t-68-23q-62-31-122-42t-127-20q-23-3-44-8t-37-17-25-28-10-45q0-31 8-60t9-61q0-22-3-43t-4-45q0-51 24-75t70-38q22-7 38-19t30-26 26-31 27-34q3-4 3-9 0-12-1-23t-2-24q0-68 21-137t55-135 76-127 83-113q58-74 87-154t30-174q0-40-4-79t-4-80q0-80 18-144t56-110 99-69 146-25q101 0 161 40t92 105 41 145 10 162v17q0 41 1 73t8 63 21 62 42 67q48 63 98 127t91 132 67 142 27 160q0 69-21 133zM861 386q14 0 23 7t14 19 7 24 2 25q0 10-3 16t-9 11-12 10-12 12q-7 11-19 18t-23 15-20 17-9 25q0 9 9 15 24 16 34 39t22 45 31 37 59 15h6q40-2 76-21t72-40q5-3 13-6t13-7l73-57q2-7 3-13t2-14q0-11-5-18t-12-12-17-7-18-5q-24-5-45-18t-44-20q-4-1-6-6t-4-12-1-13-1-10q0-12 2-25t9-25 16-18 26-7q31 0 45 23t15 50q0 13-5 23t-5 22q0 8 5 11t13 4q23 0 30-11t8-33q0-23-4-51t-16-52-31-40-49-16q-52 0-75 26t-24 78q0 15 2 30t2 30q0 5-1 5t-6-2-12-5-15-5-16-2q-2 0-10 1t-16 1q-14 0-14-5 0-15-2-38t-10-46-20-38-36-16q-17 0-29 11t-21 26-11 34-4 32q0 6 2 21t7 31 11 28 14 12q5 0 13-7t8-13q0-3-3-4t-6-1q-7 0-12-7t-8-16-6-19-2-15q0-20 9-36t33-17zM643 1943q24 0 47-4t43-16 30-31 12-48q0-18-6-35t-17-32q-12-19-26-36t-26-36q-19-28-37-55t-36-57q-16-26-30-52t-34-51q-12-15-28-27t-37-12q-22 0-38 14t-33 34-38 39-54 30q-26 8-40 22t-14 43q0 20 3 40t4 40q0 27-8 51t-8 47q0 27 19 38t43 15q30 5 57 8t53 9 52 13 55 22q6 3 18 7t27 9 28 8 19 3zm366-120q28 0 60-6t64-17 61-26 52-33q2-2 4-6t3-8v-1q6-22 10-48t7-53 7-53 7-51q4-27 10-52t16-47 27-39 44-32v-2l-1-3q0-9 6-19t15-21 19-17 21-10q-6-24-13-48t-12-48q-6-36-12-61t-14-46-23-41-37-48q-11-13-15-20t-6-25q-1-7-6-26t-13-44-18-52-22-49-22-37-21-15q-24 0-57 19t-70 42-72 43-63 19q-30 0-55-19t-46-44-35-43-21-20q-8 0-9 12t-1 27v17q0 6-2 8-11 23-24 45t-26 45-20 47-8 50q0 15 2 30t10 29l-2 4q-8 11-17 20t-17 21q-37 55-52 119t-16 130q0 17 2 34t2 34q0 5-1 11t-1 11q17 1 40 14t50 33 51 46 46 51 32 50 13 41q0 26-16 42t-40 26q17 30 42 51t54 34 63 19 66 6zm401 142q21 0 41-6 31-9 57-26t48-42q27-31 57-54t68-42q17-8 33-15t33-17q11-7 21-17t10-24q0-8-3-15t-9-15q-15-21-26-39t-19-38-16-40-14-45q-1-5-7-13-21-30-59-30-18 0-34 9t-34 19-36 19-38 9q-20 0-33-10t-22-27-15-33-12-33q-8 13-18 24t-16 26q-13 30-16 67-5 58-13 113t-27 110q-5 16-8 35t-4 37q0 49 30 81t81 32z" />
            </svg>
        }
    }
}
