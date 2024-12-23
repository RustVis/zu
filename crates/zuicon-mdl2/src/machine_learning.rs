// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct MachineLearning {}

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

impl Component for MachineLearning {
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
                data-icon={ "MachineLearning" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1968 1095q38 36 59 84t21 101q0 53-20 99t-55 82-81 55-100 20q-21 0-42-3l-471 235q1 6 1 12t0 12q0 53-20 99t-55 82-81 55-100 20q-52 0-98-20t-82-54-56-81-21-98v-13q0-6 2-14l-471-235q-21 3-42 3-53 0-99-20t-82-55-55-81-20-100q0-71 36-131t99-94l175-642q-54-69-54-157 0-53 20-99t55-82 81-55T512 0q69 0 128 34t94 94h580q35-60 94-94t128-34q53 0 99 20t82 55 55 81 20 100q0 42-13 81t-39 73l228 685zm-432 185q0-55 22-105t64-86l-449-337-663 497 2 31h1024zM512 512q-47 0-92-17l-144 530q55 5 103 31t82 70l606-454-350-262q-37 48-90 75t-115 27zm1241 515q8-2 18-2t19-1h10q5 0 10 1l-177-532q-49 19-97 19-20 0-39-3l-217 163 473 355zm-217-899q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10zm-256 128H768l-2 31 407 305 193-145q-42-36-64-86t-22-105zM512 128q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10zM128 1280q0 27 10 50t27 40 41 28 50 10q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50zm896 640q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10zm0-384q63 0 119 29t92 82l375-187q-11-12-21-25t-19-27H478q-8 14-18 27t-22 25l375 187q35-52 91-81t120-30zm768-128q27 0 50-10t40-27 28-41 10-50q0-33-16-62t-44-46l-1 1-1-1 1-1q-32-19-67-19-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10z" />
            </svg>
        }
    }
}
