// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct UnsyncOccurence {}

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

impl Component for UnsyncOccurence {
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
                data-icon={ "UnsyncOccurence" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1933 1843l-90 90-235-234q-119 103-267 162t-317 59q-119 0-231-30t-212-87-182-136-143-181v306H128v-512h512v128H359q51 89 122 160t158 120 183 77 202 27q141 0 266-48t228-136L439 530q-63 73-106 160t-63 187l-126-25q23-119 76-222t129-190L115 205l90-90 1728 1728zM1024 256q-97 0-187 24t-171 67l-94-95q101-60 214-92t238-32q118 0 230 30t211 87 183 137 144 180V256h128v512h-512V640h281q-51-88-122-159t-158-121-184-77-201-27zm773 1221l-96-96q26-49 46-100t31-109l125 24q-14 77-41 146t-65 135z" />
            </svg>
        }
    }
}
