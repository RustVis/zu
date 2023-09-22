// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod column;

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::flex_direction::FlexDirection;
use crate::styles::flex_wrap::FlexWrap;
pub use column::Column;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(12)]
    pub columns: i32,

    #[prop_or_default]
    pub column_spacing: Option<i32>,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or(false)]
    pub container: bool,

    #[prop_or(FlexDirection::Row)]
    pub direction: FlexDirection,

    #[prop_or(false)]
    pub item: bool,

    #[prop_or_default]
    pub lg: Column,

    #[prop_or_default]
    pub md: Column,

    #[prop_or_default]
    pub sm: Column,

    #[prop_or_default]
    pub row_spacing: Option<i32>,

    #[prop_or_default]
    pub spacing: Option<i32>,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub wrap: FlexWrap,

    #[prop_or_default]
    pub xl: Column,

    #[prop_or_default]
    pub xs: Column,

    #[prop_or(false)]
    pub zero_min_width: bool,
}

#[function_component(Grid)]
pub fn grid(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
