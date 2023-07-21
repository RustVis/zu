// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod row_height;
mod variant;

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

pub use row_height::RowHeight;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(2)]
    pub cols: i32,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or(4)]
    pub gap: i32,

    #[prop_or_default]
    pub row_height: RowHeight,

    #[prop_or_default]
    pub style: AttrValue,

    pub variant: Variant,
}

#[function_component(ImageList)]
pub fn image_list(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
