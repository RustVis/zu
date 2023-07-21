// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod text_align;
mod variant;

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};
use zu_util::prop::ToAttr;

use crate::styles::color::Color;
use crate::styles::CssClass;
pub use text_align::TextAlign;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub align: TextAlign,

    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub color: Color,

    #[prop_or_default]
    pub component: AttrValue,

    /// If true, the text will have a bottom margin.
    #[prop_or(false)]
    pub gutter_bottom: bool,

    #[prop_or_default]
    pub href: AttrValue,

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
        props.align.css_class(),
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
        props.classes.clone(),
    );

    html! {
       <@{component.to_owned()}
            aria_label={props.aria_label.to_attr()}
            class={cls}
            href={props.href.to_attr()}
            style={props.style.to_attr()}>
            {for props.children.iter()}
       </@>
    }
}
