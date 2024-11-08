// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct JoinOnlineMeeting {}

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

impl Component for JoinOnlineMeeting {
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
                data-icon={ "JoinOnlineMeeting" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M918 1881q-10 38-15 77t-7 80q-128-15-244-60t-215-115-180-162-138-200-88-228-31-249q0-141 36-272t103-245 160-207 208-160T751 37t273-37q141 0 271 36t245 104 207 160 161 208 103 244 37 272h-71q-14-23-29-45t-35-42q-6-87-31-169H657q-8 64-12 127t-5 129q0 65 4 128t13 128h367q0 32 5 64t13 64H679q12 57 32 125t50 133 69 122 88 93zm915-1241q-39-81-92-152t-120-130-142-105-162-75q36 50 64 106t51 115 39 120 28 121h334zm-809-512q-50 0-92 27t-78 71-64 99-51 113-37 110-23 92h690q-8-39-23-92t-37-110-50-112-64-100-79-71-92-27zm-292 49q-85 29-162 74T427 356 307 487t-92 153h334q11-55 27-117t40-124 52-119 64-103zm-604 847q0 133 37 256h363q-8-64-12-127t-4-129q0-65 4-128t12-128H165q-37 123-37 256zm85 384q39 81 92 152t120 131 144 104 162 75q-36-50-64-106t-51-115-39-120-29-121H213zm1549 181q65 33 118 81t90 108 58 128 20 142h-128q0-79-30-149t-82-122-123-83-149-30q-80 0-149 30t-122 82-83 123-30 149h-128q0-73 20-141t57-129 90-108 118-81q-75-54-116-135t-41-174q0-79 30-149t82-122 122-83 150-30q79 0 149 30t122 82 83 123 30 149q0 92-41 173t-117 136zm-226-53q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 53 20 99t55 82 81 55 100 20z" />
            </svg>
        }
    }
}
