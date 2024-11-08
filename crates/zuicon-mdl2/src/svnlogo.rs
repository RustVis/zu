// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Svnlogo {}

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

impl Component for Svnlogo {
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
                data-icon={ "SVNLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M531 891q0-14 11-28t30-29 40-27 45-25 43-21 33-15l9-4q140-62 287-105t296-80q180-44 360-79t363-62v410q-124 5-246 16t-246 25l-33 3q-131 14-259 27t-259 25q-82 7-164 13t-164 7q-11 0-35-1t-49-6-43-16-19-28zm1517-635q-235 57-467 113t-467 117q-280 72-563 157T0 838V256h2048zm-401 912q0 21-24 44t-57 43-65 35-48 22q-60 27-135 52t-155 48-156 43-143 36q-215 54-429 95T0 1658v-436q51-5 102-6t103-6q153-12 304-27t304-31q76-8 157-17t163-16 163-13 157-5q15 0 47 1t65 8 57 20 25 38zM89 1510q19 0 19-19t-19-19q-8 0-13 5t-6 14q0 8 5 13t14 6zm0-128q19 0 19-19t-19-19q-8 0-13 5t-6 14q0 8 5 13t14 6zm71 128q19 0 19-19t-19-19q-19 0-19 19t19 19zm0-128q19 0 19-19t-19-19q-19 0-19 19t19 19zM0 1786q255-35 505-91t500-121q265-67 526-144t517-176v532H0z" />
            </svg>
        }
    }
}
