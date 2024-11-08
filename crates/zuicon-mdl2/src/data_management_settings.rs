// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct DataManagementSettings {}

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

impl Component for DataManagementSettings {
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
                data-icon={ "DataManagementSettings" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 1796q48 0 98-3 25 66 62 123-40 4-80 6t-80 2q-134 0-262-22-37-6-84-17t-96-29-96-41-84-55-59-69-23-87V452q0-45 19-80t51-66q34-34 81-60t103-45 115-33 119-21 114-11 102-4q72 0 159 8t175 27 167 54 133 85q31 29 50 66t20 80v572h-128V642q-60 37-131 62t-147 40-152 21-146 7q-69 0-145-6t-153-22-147-40-131-62v962q0 20 14 37t29 28q47 37 114 61t142 39 147 21 130 6zm0-1536q-51 0-106 3t-112 12-110 22-101 33q-15 6-40 18t-49 29-41 35-17 40q0 8 3 15t8 14q22 32 63 57t92 43 108 30 113 19 104 11 85 3q35 0 84-3t105-10 112-20 109-30 91-42 64-58q5-7 8-14t3-15q0-21-17-40t-41-35-49-28-40-19q-47-19-101-32t-110-22-111-12-107-4zm954 1279q6 30 6 61t-6 61l124 51-49 119-124-52q-35 51-86 86l52 124-119 49-51-124q-30 6-61 6t-61-6l-51 124-119-49 52-124q-51-35-86-86l-124 52-49-119 124-51q-6-30-6-61t6-61l-124-51 49-119 124 52q35-51 86-86l-52-124 119-49 51 124q30-6 61-6t61 6l51-124 119 49-52 124q51 35 86 86l124-52 49 119-124 51zm-314 253q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75q0 40 15 75t41 61 61 41 75 15z" />
            </svg>
        }
    }
}
