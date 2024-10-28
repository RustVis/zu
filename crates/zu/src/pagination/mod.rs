// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

mod item_type;
mod variant;

use yew::{function_component, html, AttrValue, Callback, Classes, Event, Html, Properties};

use crate::pagination_item::StandardColor;
use crate::styles::shape_variant::ShapeVariant;
use crate::styles::size::Size;
pub use item_type::ItemType;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(1)]
    pub boundary_count: i32,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub color: StandardColor,

    #[prop_or(1)]
    pub count: i32,

    #[prop_or(1)]
    pub default_page: i32,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub get_item_aria_label: Option<Callback<(ItemType, i32, bool), String>>,

    #[prop_or(false)]
    pub hide_next_button: bool,

    #[prop_or(false)]
    pub hide_previous_button: bool,

    #[prop_or_default]
    pub on_change: Option<Callback<(Event, i32)>>,

    #[prop_or_default]
    pub page: i32,

    #[prop_or_default]
    pub shape: ShapeVariant,

    #[prop_or(false)]
    pub show_first_button: bool,

    #[prop_or(false)]
    pub show_last_button: bool,

    #[prop_or(1)]
    pub sibling_count: i32,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(Pagination)]
pub fn pagination(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
