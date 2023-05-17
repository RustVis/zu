// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Html, Properties};

use crate::styles::shape_variant::ShapeVariant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// Used in combination with src or srcSet to provide an alt attribute for the rendered img element.
    #[prop_or_default]
    pub alter: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or(false)]
    pub color_default: bool,

    #[prop_or_default]
    pub component: AttrValue,

    // TODO(Shaohua): Add img_props
    //pub img_props: AttrValue,
    /// The sizes attribute for the img element.
    #[prop_or_default]
    pub sizes: AttrValue,

    /// The src attribute for the img element.
    #[prop_or_default]
    pub src: AttrValue,

    /// The srcSet attribute for the img element.
    #[prop_or_default]
    pub src_set: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: ShapeVariant,
}

#[function_component(Avatar)]
pub fn avatar(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
