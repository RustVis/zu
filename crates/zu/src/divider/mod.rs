// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod orientation;
mod text_align;
mod variant;

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};
use zu_util::prop::ToAttr;

use crate::styles::orientation::Orientation;
use crate::styles::CssClass;
pub use text_align::TextAlign;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// Absolutely position the element.
    #[prop_or(false)]
    pub absolute: bool,

    #[prop_or_default]
    pub classes: Classes,

    /// Default value is `vr` or `hr` or `div`.
    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or_default]
    pub children: Children,

    /// If true, a vertical divider will have the correct height when used in flex container.
    #[prop_or(false)]
    pub flex_item: bool,

    /// If true, the divider will have a lighter color.
    #[prop_or(false)]
    pub light: bool,

    #[prop_or_default]
    pub orientation: Orientation,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or(TextAlign::Center)]
    pub text_align: TextAlign,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(Divider)]
pub fn divider(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuDivider-root",
        if props.absolute {
            "ZuDivider-absolute"
        } else {
            ""
        },
        props.variant.css_class(),
        if props.light { "ZuDivider-light" } else { "" },
        if props.orientation == Orientation::Vertical {
            "ZuDivider-vertical"
        } else {
            "ZuDivider-horizontal"
        },
        if props.flex_item {
            "ZuDivider-flexItem"
        } else {
            ""
        },
        if props.children.is_empty() {
            ""
        } else {
            "ZuDivider-withChildren"
        },
        if props.children.is_empty() {
            ""
        } else {
            orientation::css_class(props.orientation)
        },
        if props.orientation == Orientation::Horizontal {
            props.text_align.css_class()
        } else {
            ""
        },
        props.classes.clone(),
    );

    let wrapper_cls = classes!(
        "ZuDivider-wrapper",
        if props.orientation == Orientation::Horizontal {
            "ZuDivider-wrapperHorizontal"
        } else {
            "ZuDivider-wrapperVertical"
        },
    );

    let component = if props.component.is_empty() {
        if props.children.is_empty() {
            match props.orientation {
                Orientation::Vertical => "vr".to_owned(),
                Orientation::Horizontal => "hr".to_owned(),
            }
        } else {
            "div".to_owned()
        }
    } else {
        props.component.as_str().to_owned()
    };

    let role = if component != "hr" && component != "vr" {
        Some("separator")
    } else {
        None
    };
    let style = props.style.to_attr();

    if props.children.is_empty() {
        html! {
            <@{component} class={root_cls} style={style} role={role} />
        }
    } else {
        html! {
            <@{component} class={root_cls} style={style} role={role}>
                <span class={wrapper_cls}>
                    {for props.children.iter()}
                </span>
            </@>
        }
    }
}
