// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// The component used for the root node.
    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: String,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Box)]
pub fn r#box(props: &Props) -> Html {
    let cls = classes!("ZuBox-root", &props.classes);
    let component = if props.component.is_empty() {
        "div"
    } else {
        props.component.as_str()
    };
    let style = if props.style.is_empty() {
        None
    } else {
        Some(props.style.clone())
    };

    html! {
        <@{component.to_owned()} class={cls} style={style}>
            {for props.children.iter()}
        </@>
    }
}
