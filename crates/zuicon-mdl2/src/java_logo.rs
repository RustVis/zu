// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct JavaLogo {}

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

impl Component for JavaLogo {
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
                data-icon={ "JavaLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M768 1584q-7 4-15 12t-8 16q0 10 15 17t40 12 54 8 56 4 50 2 32 1q65 0 129-8t127-21q26 17 54 30t58 25q-86 37-178 54t-186 17q-13 0-42-2t-66-6-75-11-71-19-52-27-21-37q0-14 11-25t28-19 33-14 27-9zm545-172q17 17 39 30t45 22q-107 31-217 46t-223 16q-12 0-42-1t-70-4-80-10-76-17-57-24-23-34q0-14 13-26t32-23 36-17 29-10q-9 6-18 18t-10 24q0 11 9 18t21 11 24 5 20 3q49 6 98 8t99 3q89 0 176-10t175-28zm185-994q-23 13-57 33t-71 44-76 54-68 60-50 63-19 64q0 37 18 68t39 60 40 60 18 67q0 32-16 63t-40 57-53 48-57 36q26-21 45-55t19-69q0-35-20-65t-44-62-45-72-20-93q0-49 22-92t60-79 86-67 99-55 100-41 90-27zm164 1332q11 5 25 15t15 25q0 23-31 42t-80 36-107 28-111 21-95 14-57 7q-140 12-279 12-140 0-280-10t-278-35q-9-1-26-4t-33-10-29-16-13-23q0-17 16-32t43-29 58-25 62-18 55-12 40-4q14 0 27 2t26 10h-6q-12 0-44 5t-65 15-58 23-26 29q0 8 7 13t14 8q26 13 66 22t88 16 99 10 101 6 91 3 73 1h55q39 0 94-3t117-5 128-10 124-15 106-21 74-28 28-36q0-16-14-27zm-281-498q-127 33-259 44t-264 11h-46q-35 0-81-3t-96-7-92-12-69-19-28-28q0-6 4-11t10-10 12-8 11-6q32-17 71-31t79-24 82-15 78-6h13q-9 2-28 7t-42 12-49 15-46 18-34 18-14 19q0 10 15 16t40 11 53 7 57 4 51 1 36 1h6q158 0 313-13t311-41q-24 11-48 23t-46 27zm92 249q32-16 72-41t77-58 60-70 25-80q0-25-10-47t-29-38-41-25-48-9q-17 0-34 3t-33 11q7-12 20-19t29-12 32-7 30-2q31 0 61 10t54 29 38 46 15 61q0 37-16 68t-43 57-61 48-70 37-71 29-64 19l7-10zm-433-377q-35-31-82-75t-89-94-71-104-30-104q0-57 36-106t90-95 117-94 117-100 90-117 36-142q0-48-12-93 13 14 23 32t17 38 10 42 4 39q0 60-24 115t-61 104-83 92-88 78q-29 24-58 51t-54 59-39 68-15 76q0 47 19 89t46 82 55 79 46 80zm752 730q-3 35-31 61t-73 47-103 34-122 24-126 15-120 8-99 4-68 1q-25 0-67-1t-92-3-102-7-100-11-83-16-52-23q103 17 207 23t210 6q129 0 259-9t257-34q12-2 38-8t58-14 66-19 64-23 51-26 28-29z" />
            </svg>
        }
    }
}
