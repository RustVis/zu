// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct DockerLogo {}

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

impl Component for DockerLogo {
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
                data-icon={ "DockerLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M780 848v149h-14V848h14zm-53 0v149h-14V848h14zm-197-57V641h15v150h-15zm144-150v150h-15V641h15zM504 791V641h14v150h-14zm-27 0V641h15v150h-15zm-26 0V641h14v150h-14zm303 57v149h-15V848h15zm-54 0v149h-14V848h14zm0-207v150h-14V641h14zm54 0v150h-15V641h15zm-27 0v150h-14V641h14zm-53 207v149h-15V848h15zm106-207v150h-14V641h14zm27 207v149h-14V848h14zM270 997V848h15v149h-15zm-26 0V848h14v149h-14zm53 0V848h14v149h-14zm260-206V641h14v150h-14zM377 997V848h14v149h-14zm207 0V848h14v149h-14zm-260 0V848h14v149h-14zm26 0V848h15v149h-15zm101 0V848h14v149h-14zm133-206V641h14v150h-14zM477 997V848h15v149h-15zm80 0V848h14v149h-14zm-27 0V848h15v149h-15zm-26 0V848h14v149h-14zm303-356v150h-14V641h14zm129-207v150h-14V434h14zm-27 0v150h-14V434h14zm-26 207v150h-15V641h15zm0-207v150h-15V434h15zm133 0v150h-15V434h15zm-53 0v150h-15V434h15zm26 0v150h-14V434h14zm236 414v149h-15V848h15zm-107 0v149h-14V848h14zm-26 0v149h-15V848h15zm106 0v149h-14V848h14zm-53 0v149h-15V848h15zm26 0v149h-14V848h14zm-262 0v149h-14V848h14zm27 0v149h-14V848h14zm27 0v149h-15V848h15zm-80 0v149h-15V848h15zm26-207v150h-14V641h14zm107 0v150h-15V641h15zm-27 207v149h-14V848h14zm27 0v149h-15V848h15zm-27-207v150h-14V641h14zm-26 0v150h-15V641h15zm-27 0v150h-14V641h14zm-324 678q-7 5-7 13 0 6 4 10t10 5q10 0 13-8 3 6 3 14 0 15-10 25t-26 11q-15 0-25-10t-11-26q0-15 10-25t26-11q5 0 13 2zm1435-454l-16 29q-22 40-52 67t-66 45-76 24-83 8h-13q-6 0-14-1-69 169-175 292t-243 205-297 120-338 39q-134 0-256-39t-216-114-148-186-55-254q0-23 1-45t6-45h191V803h209V596h415V389h244v414h207v207h105q57 0 112-14t105-42q-27-34-39-76t-12-86q0-60 17-105t58-90q28 23 54 47t48 52 36 60 19 71q23-8 47-11t50-3q52 0 91 13t84 39zM438 629v174h174V629H438zm211 724q0-21-14-35t-36-15q-21 0-35 14t-15 36q0 20 15 35t35 15q20 0 35-15t15-35zM438 836v174h174V836H438zm-207 0v174h175V836H231zm444 826h28q14 0 29-2-77-36-136-90t-94-133q-40 11-81 16t-83 8l-2 1q-25 1-49 1t-49 1q-19 0-37-1t-38-2q55 54 112 92t121 63 132 35 147 11zm146-652V836H647v174h174zm0-207V629H647v174h174zm209 207V836H856v174h174zm0-207V629H856v174h174zm0-207V422H856v174h174zm209 414V836h-174v174h174z" />
            </svg>
        }
    }
}
