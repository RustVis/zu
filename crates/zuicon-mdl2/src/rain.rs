// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Rain {}

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

impl Component for Rain {
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
                data-icon={ "Rain" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M850 1827q14 30 14 61 0 33-12 62t-35 51-51 34-62 13q-33 0-62-12t-51-35-34-51-13-62q0-31 14-61l146-291 146 291zm768 0q14 30 14 61 0 33-12 62t-35 51-51 34-62 13q-33 0-62-12t-51-35-34-51-13-62q0-31 14-61l146-291 146 291zm-384-256q14 30 14 61 0 33-12 62t-35 51-51 34-62 13q-33 0-62-12t-51-35-34-51-13-62q0-31 14-61l146-291 146 291zm527-535q63 17 115 52t91 85 60 109 21 126q0 66-22 126t-61 110-93 85-118 51l-60-120q48-6 89-27t72-56 47-77 18-92q0-55-20-102t-57-81-84-53-102-20q-12-82-51-152t-97-122-134-81-159-29q-77 0-146 25t-127 69-98 106-61 135q-44-38-97-58t-111-21q-66 0-124 25t-102 68-69 102-25 125q0 53 17 102t47 89 73 70 94 45l-57 114q-67-23-122-65t-96-97-62-120-22-138q0-93 35-174t96-142 142-96 175-36q74 0 147 25 39-65 92-117t117-88 136-56 148-20q93 0 178 29t158 81 126 125 83 161z" />
            </svg>
        }
    }
}
