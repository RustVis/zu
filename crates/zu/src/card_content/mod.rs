// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};
use zu_util::prop;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

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
    let root_cls = classes!("ZuCardContent-root", props.classes.as_str().to_owned(),);
    html! {
        <@{component.to_owned()} class={root_cls} style={prop::attr_optional(&props.style)}>
        </@>
    }
}
