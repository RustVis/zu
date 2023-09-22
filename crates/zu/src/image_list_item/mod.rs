// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(1)]
    pub cols: i32,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or(1)]
    pub rows: i32,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(ImageListItem)]
pub fn image_list_item(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
