// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct MyNetwork {}

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

impl Component for MyNetwork {
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
                data-icon={ "MyNetwork" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1836 517q42 87 63 184t21 195q0 131-36 252t-102 227-162 190-212 141v86H768v128h256v128H384v-128h256v-128H0V896h128q0-124 32-238t90-214 140-181 181-140 214-91 239-32q130 0 252 36t227 102 190 162 141 212h2v5zm-147-5q-73-127-187-217t-254-134q48 78 77 169t47 182h317zm103 384q0-132-44-256h-356q16 128 16 255t0 257h340q44-124 44-256zm-512 0q0-65-4-128t-12-128H784q-8 64-12 127t-4 129h512zm-256-763q-29 0-55 21t-49 56-41 78-33 85-25 79-15 60h436q-5-23-15-60t-24-79-33-85-41-77-49-56-56-22zm-224 28q-140 43-254 133T359 512h317q17-90 46-181t78-170zM300 640q-44 124-44 256h384q0-65 4-128t12-128H300zM128 1664h1152v-640H128v640zm1280-103q88-51 159-122t123-159h-282v281z" />
            </svg>
        }
    }
}
