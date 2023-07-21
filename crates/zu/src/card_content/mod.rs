// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};
use zu_util::prop::ToAttr;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(CardContent)]
pub fn card_content(props: &Props) -> Html {
    let component = if props.component.is_empty() {
        "div"
    } else {
        props.component.as_str()
    };
    let root_cls = classes!("ZuCardContent-root", props.classes.clone());

    html! {
        <@{component.to_owned()} class={root_cls}
            style={props.style.to_attr()}>
        </@>
    }
}
