// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod item_type;

use yew::{function_component, html, AttrValue, Classes, Html, Properties};

use crate::pagination::Variant;
use crate::styles::color::StandardColor;
use crate::styles::shape_variant::ShapeVariant;
use crate::styles::size::Size;
pub use item_type::ItemType;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub color: StandardColor,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub page: Option<Html>,

    #[prop_or(false)]
    pub selected: bool,

    #[prop_or_default]
    pub shape: ShapeVariant,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(PaginationItem)]
pub fn pagination_item(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
