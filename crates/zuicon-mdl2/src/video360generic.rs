// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Video360generic {}

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

impl Component for Video360generic {
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
                data-icon={ "Video360Generic" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 0q154 0 298 44t267 126 221 197 158 258q34 81 56 174t23 181q0 60-3 116t-13 110-24 107-39 110q-60 142-157 257t-220 197-268 126-299 45q-154 0-298-44t-268-126-221-197-157-258q-24-56-39-109t-24-108-12-110-4-116q0-88 22-181t57-174q60-142 157-257t221-197T726 45t298-45zm478 692q14 82 20 165t6 167q0 100-9 199 75-18 148-46t140-70q14-9 33-24t36-33 30-37 13-38q0-11-1-23t-3-22q-6-30-34-58t-65-53-74-42-63-29q-43-17-87-31t-90-25zm355 0q-37-92-93-173t-127-147-155-118-179-82q62 84 104 183t66 200q101 21 197 53t187 84zm-854-564h-16q-8 0-16 2-6 1-20 8t-19 11q-29 17-54 42t-46 53-40 60-32 61q-20 41-35 83t-28 86q81-11 163-16t164-6q82 0 164 5t163 17q-13-44-28-86t-35-83q-14-30-32-61t-39-59-47-54-54-42q-5-4-19-11t-20-8q-8-2-16-2t-16 0h-42zm-258 44q-94 31-178 82T411 371 284 519t-93 173q91-51 187-83t197-54q24-101 66-200t104-183zM145 1197q23 119 77 225t132 195 177 155 215 104q-58-76-96-165t-65-181q30 6 60 10t62 9q14 44 34 92t46 95 59 87 72 68q4 3 11 7t15 8 15 8 12 5q6 1 12 1t14 0h64q8 0 16-2t22-9 22-12q41-26 75-66t61-88 48-96 35-94q41-126 58-261t18-268q0-90-7-179t-25-177q-90-14-179-21t-181-7q-91 0-180 7t-180 21q-17 88-24 177t-8 179v29q0 14 2 30-29-5-57-12t-56-16q-1-8-1-15t0-16q0-84 6-167t20-165q-45 11-89 25t-88 31q-26 11-63 29t-74 42-64 52-35 59q-2 10-3 22t-1 23q0 5 4 16t7 16q15 29 42 54t59 46 65 37 63 28q74 30 156 50t166 34 169 18 164 6h37l-146-147 90-90 301 301-301 301-90-90 147-147q-73 0-152-3t-160-13-164-24-159-39-149-56-133-76zm1158 679q115-38 214-104t178-155 131-195 77-226q-43 32-92 57t-100 45-105 35-105 26q-11 66-28 135t-41 135-56 130-73 117z" />
            </svg>
        }
    }
}
