// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AccessLogo {}

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

impl Component for AccessLogo {
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
                data-icon={ "AccessLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1280 64q137 0 274 15 24 3 69 9t101 18 112 28 105 39 77 51 30 64v1472q0 35-30 63t-77 51-104 39-113 28-100 19-70 9q-137 15-274 15-138 0-274-15-24-2-69-9t-101-18-112-28-105-39-77-51-30-64v-224H85q-35 0-60-25t-25-60V597q0-35 25-60t60-25h427V288q0-35 30-63t77-51 104-39 113-28 100-18 70-10q136-15 274-15zm0 128q-65 0-145 4t-162 16-161 29-138 47q62 29 139 47t159 29 161 15 147 5q67 0 146-4t161-15 159-29 140-48q-60-28-138-46t-160-30-163-15-145-5zM403 1207h217l51 153h171L597 688H430l-248 672h170l51-153zm1517 532v-303q-66 29-147 48t-168 30-170 17-155 5q-73 0-145-4t-145-13q-23 17-51 17H640v203q29 23 78 41t108 31 125 21 126 14 115 8 88 2q35 0 88-2t114-7 127-14 124-22 109-31 78-41zm0-446V924q-66 29-147 48t-168 30-170 17-155 5q-64 0-128-3t-128-10v384q64 7 128 10t128 3q37 0 90-2t113-7 125-14 123-22 108-30 81-40zm0-512V412q-66 29-147 48t-168 30-170 17-155 5q-71 0-155-5t-170-16-167-31-148-48v100h299q35 0 60 25t25 60v286q64 7 128 10t128 3q37 0 90-2t113-7 125-14 123-22 108-30 81-40zM442 1077l69-209 67 209H442z" />
            </svg>
        }
    }
}
