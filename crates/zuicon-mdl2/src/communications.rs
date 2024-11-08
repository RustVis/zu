// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Communications {}

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

impl Component for Communications {
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
                data-icon={ "Communications" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 896q52 0 99 20t81 55 55 82 21 99q0 53-20 99t-55 81-82 55-99 21q-53 0-99-20t-81-55-55-81-21-100q0-52 20-99t55-81 81-55 100-21zm0 384q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10zm0-1152q141 0 271 36t245 104 207 160 161 208 103 244 37 272q0 102-20 200t-58 191-94 177-128 156l-90-90q63-63 111-136t82-155 51-167 18-176q0-124-32-238t-90-214-140-181-181-140-214-91-239-32q-124 0-238 32t-214 90-181 140-140 181-91 214-32 239q0 89 17 175t51 168 82 154 112 137l-90 90q-72-72-128-156t-94-176-58-191-20-201q0-141 36-271t104-245 160-207 208-161 244-103 272-37zm0 384q133 0 249 50t204 137 137 203 50 250q0 128-48 245t-139 208l-91-91q72-72 111-166t39-196q0-106-40-199t-110-162-163-110-199-41q-106 0-199 40T663 790 553 953t-41 199q0 102 39 196t111 166l-91 91q-90-91-138-208t-49-245q0-133 50-249t137-204 203-137 250-50z" />
            </svg>
        }
    }
}
