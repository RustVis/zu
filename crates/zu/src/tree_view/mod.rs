// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Callback, Children, Classes, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub default_collapse_icon: Option<Html>,

    #[prop_or_default]
    pub default_end_icon: Option<Html>,
    //TODO(Shaohua): Add defaultExpanded
    #[prop_or_default]
    pub default_expand_icon: Option<Html>,

    #[prop_or_default]
    pub default_parent_icon: Option<Html>,
    //TODO(Shaohua): Add defaultSelected
    #[prop_or(false)]
    pub disabled_items_focusable: bool,

    #[prop_or(false)]
    pub disable_selection: bool,
    //TODO(Shaohua): Add expanded
    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or(false)]
    pub multi_select: bool,

    #[prop_or_default]
    pub on_node_focus: Option<Callback<()>>,

    #[prop_or_default]
    pub on_node_select: Option<Callback<()>>,

    #[prop_or_default]
    pub on_node_toggle: Option<Callback<()>>,
    //TODO(Shaohua): Add selected
    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(TreeView)]
pub fn tree_view(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
