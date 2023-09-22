// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::sort_direction::SortDirection;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub active: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub direction: SortDirection,

    #[prop_or(false)]
    pub hide_sort_icon: bool,
    // TODO(Shaohua): Add icon component.
    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(TableSortLabel)]
pub fn table_sort_label(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
