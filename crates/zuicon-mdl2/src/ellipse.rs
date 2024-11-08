// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Ellipse {}

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

impl Component for Ellipse {
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
                data-icon={ "Ellipse" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 256q131 0 268 27t264 85 233 144 175 206q41 71 62 147t22 159q0 82-21 158t-63 148q-68 119-174 206t-233 144-264 84-269 28q-131 0-268-27t-264-85-233-144-175-206q-41-71-62-147T0 1024q0-82 21-158t63-148q68-119 174-206t233-144 264-84 269-28zm0 1408q84 0 169-11t167-36 159-60 146-87q54-40 101-88t81-105 53-120 20-133q0-70-19-133t-54-119-81-105-101-89q-68-50-145-86t-160-61-167-35-169-12q-84 0-169 11t-167 36-159 60-146 87q-54 40-101 88t-81 105-53 120-20 133q0 70 19 133t54 119 81 105 101 89q68 50 145 86t160 61 167 35 169 12z" />
            </svg>
        }
    }
}
