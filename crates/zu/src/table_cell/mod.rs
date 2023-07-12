// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod text_align;
mod variant;

use yew::{function_component, html, AttrValue, Children, Html, Properties};

use crate::styles::size::Size;
use crate::styles::sort_direction::SortDirection;
use crate::table::Padding;
pub use text_align::TextAlign;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub align: TextAlign,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub padding: Padding,

    #[prop_or_default]
    pub scope: AttrValue,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub sort_direction: Option<SortDirection>,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(TableCell)]
pub fn table_cell(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
