// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct LifesaverLock {}

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

impl Component for LifesaverLock {
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
                data-icon={ "LifesaverLock" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1664 1024q53 0 99 20t82 55 55 81 20 100v128h128v640h-768v-640h128v-128q0-53 20-99t55-82 81-55 100-20zm-128 384h256v-128q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50v128zm384 128h-512v384h512v-384zM512 1024q0 106 40 199t110 162 163 110 199 41q32 0 64-4t64-12v131q-32 6-64 9t-64 4q-52 0-102-8t-100-25l-81 243q69 23 140 34t143 12q65 0 128-9v129q-32 4-64 6t-64 2q-141 0-272-36t-244-104-207-160-161-207-103-245-37-272q0-141 36-272t104-244 160-207 207-161T752 37t272-37q141 0 272 36t244 104 207 160 161 207 103 245 37 272q0 80-12 159-15-55-44-103t-73-86q-3-130-45-253-61 20-121 40t-122 41q6 18 11 36t9 38h-131q-22-85-69-155t-113-122-146-79-168-28q-106 0-199 40T663 662 553 825t-41 199zm512-896q-72 0-143 11t-140 35q20 61 40 121t41 122q49-17 99-25t103-8q52 0 102 8t100 25l81-243q-69-23-140-34t-143-12zM417 1226q-17-49-25-99t-8-103q0-52 8-102t25-100l-243-81q-23 69-34 140t-12 143q0 72 11 143t35 140q61-20 121-40t122-41z" />
            </svg>
        }
    }
}
