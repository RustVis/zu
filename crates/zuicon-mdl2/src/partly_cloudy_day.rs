// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PartlyCloudyDay {}

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

impl Component for PartlyCloudyDay {
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
                data-icon={ "PartlyCloudyDay" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1761 1036q64 18 116 53t91 84 59 109 21 126q0 80-30 149t-82 122-123 83-149 30H576q-93 0-174-35t-143-96-96-142-35-175q0-66 19-128t54-117 84-96 111-69q-12-49-12-102 0-93 35-174t96-143 142-96 175-35q64 0 125 18t113 51 95 81 70 106q90 3 173 33t152 82 122 124 79 157zM832 512q-66 0-124 25t-102 69-69 102-25 124q0 17 1 33t7 34q28-3 56-3 38 0 75 6t72 19q29-51 69-94t89-78 102-59 112-37q-44-66-114-103t-149-38zm832 1152q53 0 99-20t82-55 55-81 20-100q0-55-20-102t-57-81-84-53-102-20q-12-82-51-152t-97-122-134-81-159-29q-77 0-146 25t-127 69-98 106-61 135q-44-38-97-58t-111-21q-66 0-124 25t-102 68-69 102-25 125q0 66 25 124t68 102 102 69 125 25h1088zM256 896H0V768h256v128zm123-426L198 289l91-91 181 181-91 91zm517-214H768V0h128v256zm389 214l-91-91 181-181 91 91-181 181z" />
            </svg>
        }
    }
}
