// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Children, Html, Properties};

use crate::styles::text_align::TextAlign;
use crate::styles::{CssClass, CssValue};

/// Applies the theme typography styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Body1,
    Body2,
    Button,
    Caption,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Overline,
    Subtitle1,
    Subtitle2,
    Inherit,
}

impl Default for Variant {
    fn default() -> Self {
        Self::Body1
    }
}

impl Variant {
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub const fn as_component(&self) -> &'static str {
        match self {
            Self::Body1 => "p",
            Self::Body2 => "p",
            Self::Button => "span",
            Self::Caption => "span",
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
            Self::Overline => "span",
            Self::Subtitle1 => "h6",
            Self::Subtitle2 => "h6",
            Self::Inherit => "p",
        }
    }
}

impl CssClass for Variant {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Body1 => "ZuTypography-body1",
            Self::Body2 => "ZuTypography-body2",
            Self::Button => "ZuTypography-button",
            Self::Caption => "ZuTypography-caption",
            Self::H1 => "ZuTypography-h1",
            Self::H2 => "ZuTypography-h2",
            Self::H3 => "ZuTypography-h3",
            Self::H4 => "ZuTypography-h4",
            Self::H5 => "ZuTypography-h5",
            Self::H6 => "ZuTypography-h6",
            Self::Overline => "ZuTypography-overline",
            Self::Subtitle1 => "ZuTypography-subtitle1",
            Self::Subtitle2 => "ZuTypography-subtitle2",
            Self::Inherit => "",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub align: TextAlign,

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
    pub style: String,
}

#[function_component(Typography)]
pub fn typography(props: &Props) -> Html {
    let component = if props.paragraph {
        "p"
    } else {
        props.variant.as_component()
    };

    let cls = props.variant.css_class();
    let styles = vec![
        props.style.clone(),
        format!("text-align: {}", props.align.css_value()),
    ];
    let style = styles.join(";");

    html! {
       <div as={component} class={cls} style={style}>
            {props.children.clone()}
       </div>
    }
}
