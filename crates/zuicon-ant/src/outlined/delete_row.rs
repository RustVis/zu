// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct DeleteRow {}

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

impl Component for DeleteRow {
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
                data-icon={ "delete-row" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <defs><style type="text/css"></style></defs><path d="M819.8 512l102.4-122.9c2.8-3.4 2.4-8.4-1-11.3-1.4-1.2-3.2-1.9-5.1-1.9h-54.7c-2.4 0-4.6 1.1-6.1 2.9L782 466.7l-73.1-87.8c-1.5-1.8-3.8-2.9-6.1-2.9H648c-1.9 0-3.7 0.7-5.1 1.9-3.4 2.8-3.9 7.9-1 11.3L744.2 512 641.8 634.9c-2.8 3.4-2.4 8.4 1 11.3 1.4 1.2 3.2 1.9 5.1 1.9h54.7c2.4 0 4.6-1.1 6.1-2.9l73.1-87.8 73.1 87.8c1.5 1.8 3.8 2.9 6.1 2.9h55c1.9 0 3.7-0.7 5.1-1.9 3.4-2.8 3.9-7.9 1-11.3L819.8 512zM536 464H120c-4.4 0-8 3.6-8 8v80c0 4.4 3.6 8 8 8h416c4.4 0 8-3.6 8-8v-80c0-4.4-3.6-8-8-8zM452 668h-60c-3.3 0-6 2.7-6 6v166H136c-3.3 0-6 2.7-6 6v60c0 3.3 2.7 6 6 6h292c16.6 0 30-13.4 30-30V674c0-3.3-2.7-6-6-6zM136 184h250v166c0 3.3 2.7 6 6 6h60c3.3 0 6-2.7 6-6V142c0-16.6-13.4-30-30-30H136c-3.3 0-6 2.7-6 6v60c0 3.3 2.7 6 6 6z" p-id="11765"></path>
            </svg>
        }
    }
}
