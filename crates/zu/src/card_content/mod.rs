// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};
use zu_util::prop::ToAttr;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    /// Default value is `div`.
    #[prop_or(AttrValue::from("div"))]
    pub component: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(CardContent)]
pub fn card_content(props: &Props) -> Html {
    let root_cls = classes!("ZuCardContent-root", props.classes.clone());

    html! {
        <@{props.component.to_string()}
            class={root_cls}
            style={props.style.to_attr()}>
            {for props.children.iter()}
        </@>
    }
}
