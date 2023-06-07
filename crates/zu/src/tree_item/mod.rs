// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub node_id: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub collapse_icon: Option<Html>,

    // TODO(Shaohua): Add content component.
    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub end_icon: Option<Html>,

    #[prop_or_default]
    pub expand_icon: Option<Html>,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub label: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,
    // TODO(Shaohua): Add transition component.
}

#[function_component(TreeItem)]
pub fn tree_item(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
