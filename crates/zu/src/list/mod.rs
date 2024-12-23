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

    /// Default value is `ul`.
    #[prop_or(AttrValue::from("ul"))]
    pub component: AttrValue,

    /// If true, compact vertical padding designed for keyboard and mouse input
    /// is used for the list and list items.
    #[prop_or(false)]
    pub dense: bool,

    /// If true, vertical padding is removed from the list.
    #[prop_or(false)]
    pub disable_padding: bool,

    /// The content of the subheader, normally `ListSubheader`.
    #[prop_or_default]
    pub subheader: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(List)]
pub fn list(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuList-root",
        if props.disable_padding {
            ""
        } else {
            "ZuList-padding"
        },
        if props.dense { "ZuList-dense" } else { "" },
        if props.subheader.is_some() {
            "ZuList-subheader"
        } else {
            ""
        },
        props.classes.clone(),
    );

    html! {
        <@{props.component.to_string()}
            class={root_cls}
            style={props.style.to_attr()}>
            if let Some(subheader) = &props.subheader {
                {subheader.clone()}
            }
            {for props.children.iter()}
        </@>
    }
}
