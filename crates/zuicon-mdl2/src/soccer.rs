// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Soccer {}

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

impl Component for Soccer {
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
                data-icon={ "Soccer" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1261 1344H787L640 893l384-279 384 279-147 451zm-4-403l-233-169-233 169 89 275h288l89-275zM1024 0q141 0 272 36t245 103 207 160 160 208 103 245 37 272q0 141-36 272t-103 245-160 207-208 160-245 103-272 37q-141 0-272-36t-245-103-207-160-160-208-103-244-37-273q0-141 36-272t103-245 160-207 208-160T751 37t273-37zm887 895q-22-151-94-288l-135-20-35 235 167 169 97-96zm-736-754q-38-7-75-10t-76-3q-38 0-75 3t-76 10l-61 122 212 106 212-106-61-122zM231 607q-72 137-94 288l97 96 167-169-35-235-135 20zm152 1042q107 110 245 178l120-62-109-211-234-39-22 134zm405 239q117 32 236 32t236-32l-133-69 198-381 423-70 25 147q66-101 103-214t43-235l-107 106-301-306 64-425 148 22q-75-94-172-164t-210-113l67 134-384 192-384-192 67-134q-112 42-209 112T325 463l148-22 64 425-301 306-107-106q6 121 43 234t103 215l25-147 423 70 198 381-133 69zm632-61q138-68 245-178l-22-134-234 39-109 211 120 62z" />
            </svg>
        }
    }
}
