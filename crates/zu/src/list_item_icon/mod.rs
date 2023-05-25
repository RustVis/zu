// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};
use zu_util::prop;

use crate::list_item::AlignItems;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub align_items: AlignItems,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(ListItemIcon)]
pub fn list_item_icon(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuListItemIcon-root",
        if props.align_items == AlignItems::FlexStart {
            "ZuListItemIcon-alignItemsFlexStart"
        } else {
            ""
        },
        props.classes.as_str().to_owned(),
    );

    html! {
        <div class={root_cls} style={prop::attr_optional(&props.style)}>
            {for props.children.iter()}
        </div>
    }
}