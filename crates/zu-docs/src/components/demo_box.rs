// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Html, Properties};

use zu_util::prop;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(DemoBox)]
pub fn demo_box(props: &Props) -> Html {
    html! {
        <>
        <div class="demo-box" style={prop::attr_optional(&props.style)}>
            {for props.children.iter()}
        </div>

        <div class="code-root">
        </div>
        </>
    }
}
