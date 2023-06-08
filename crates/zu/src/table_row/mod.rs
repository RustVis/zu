// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Html, Properties};

use crate::table_cell::SortDirection;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub active: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub direction: SortDirection,

    #[prop_or(false)]
    pub hide_sort_icon: bool,
    // TODO(Shaohua): Add icon component.
    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(TableRow)]
pub fn table_row(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
