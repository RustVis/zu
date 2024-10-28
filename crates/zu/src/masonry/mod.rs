// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    // TODO(Shaohua): Add enum
    #[prop_or(4)]
    pub columns: i32,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or_default]
    pub default_columns: i32,

    #[prop_or_default]
    pub default_height: i32,

    #[prop_or_default]
    pub default_spacing: i32,

    // TODO(Shaohua): Add an enum
    #[prop_or(1)]
    pub spacing: i32,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Masonry)]
pub fn masonry(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
