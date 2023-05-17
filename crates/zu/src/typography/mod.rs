// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod variant;

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};

use crate::styles::color::Color;
use crate::styles::text_align::TextAlign;
use crate::styles::CssClass;

// Re-export
pub use variant::Variant;

#[must_use]
pub const fn text_align_class(align: TextAlign) -> &'static str {
    match align {
        TextAlign::Center => "ZuTypography-center",
        TextAlign::Start => "ZuTypography-start",
        TextAlign::End => "ZuTypography-end",
        TextAlign::Justify => "ZuTypography-justify",
        TextAlign::Inherit => "",
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub align: TextAlign,

    #[prop_or_default]
    pub color: Color,

    #[prop_or_default]
    pub variant: Variant,

    /// If true, the text will have a bottom margin.
    #[prop_or(false)]
    pub gutter_bottom: bool,

    /// If true, the text will not wrap, but instead will truncate with a text overflow ellipsis.
    ///
    /// Note that text overflow can only happen with block or inline-block level elements
    #[prop_or(false)]
    pub no_wrap: bool,

    /// The element will be a paragraph element or not.
    #[prop_or(false)]
    pub paragraph: bool,

    pub children: Children,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Typography)]
pub fn typography(props: &Props) -> Html {
    let mut cls_list = vec![
        "ZuTypography-root",
        props.variant.css_class(),
        text_align_class(props.align),
    ];
    if props.no_wrap {
        cls_list.push("ZuTypography-noWrap");
    }
    if props.gutter_bottom {
        cls_list.push("ZuTypography-gutterBottom");
    }
    let cls = classes!(cls_list, props.color.text_color());

    let style = if props.style.is_empty() {
        None
    } else {
        Some(props.style.clone())
    };

    let component = if props.paragraph {
        "p"
    } else {
        props.variant.as_component()
    };

    html! {
       <@{component} class={cls} style={style}>
            {for props.children.iter()}
       </@>
    }
}
