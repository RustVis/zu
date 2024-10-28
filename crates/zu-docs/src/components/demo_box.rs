// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Html, Properties};
use zu_util::prop::ToAttr;

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
        <div class="demo-box" style={props.style.to_attr()}>
            {for props.children.iter()}
        </div>

        <div class="code-root">
        </div>
        </>
    }
}
