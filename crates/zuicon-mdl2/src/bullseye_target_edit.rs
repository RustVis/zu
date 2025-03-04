// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BullseyeTargetEdit {}

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

impl Component for BullseyeTargetEdit {
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
                data-icon={ "BullseyeTargetEdit" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 1094q0 39-15 76t-43 65l-717 717-377 94 94-377 717-716q28-28 65-41t76-13q42 0 78 14t64 41 42 61 16 79zm-128 0q0-33-20-51t-52-18q-14 0-27 4t-23 14l-692 692-34 135 135-34 692-691q21-21 21-51zm-682 101q23-38 32-82t10-89q0-79-30-149t-82-122-123-83-149-30q-80 0-150 30t-122 82-82 122-30 150q0 80 30 150t82 122 122 82 150 30q45 0 88-10t83-32l-170 170h-1q-107 0-200-40t-162-109-110-163-40-200q0-106 40-199t109-163 163-110 200-40q107 0 200 40t163 110 109 164 40 200l-170 169zm-442 590l-31 125q-110-15-209-56t-184-102-154-142-117-174-74-198-27-214q0-123 32-237t90-214 141-182 181-140 214-91 238-32q148 0 282 45t246 127 195 198 130 255q-32 10-60 22t-56 34q-36-123-108-225t-169-174-214-113-246-41q-106 0-204 27t-183 78-156 120-120 155-77 184-28 204q0 95 23 185t65 171 103 150 134 122 161 87 182 46zm228-761q0 27-10 50t-27 40-41 28-50 10q-27 0-50-10t-40-27-28-41-10-50q0-27 10-50t27-40 41-28 50-10q27 0 50 10t40 27 28 41 10 50z" />
            </svg>
        }
    }
}
