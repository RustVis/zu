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
use zu_util::prop::attr_optional;

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
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub color: Color,

    #[prop_or_default]
    pub component: AttrValue,

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

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(Typography)]
pub fn typography(props: &Props) -> Html {
    let component = if !props.component.is_empty() {
        props.component.as_str()
    } else if props.paragraph {
        "p"
    } else {
        props.variant.as_component()
    };

    let cls = classes!(
        "ZuTypography-root",
        props.variant.css_class(),
        text_align_class(props.align),
        if props.no_wrap {
            "ZuTypography-noWrap"
        } else {
            ""
        },
        if props.gutter_bottom {
            "ZuTypography-gutterBottom"
        } else {
            ""
        },
        props.color.text_color(),
        props.classes.as_str().to_owned(),
    );

    html! {
       <@{component.to_owned()} class={cls} style={attr_optional(&props.style)}>
            {for props.children.iter()}
       </@>
    }
}
