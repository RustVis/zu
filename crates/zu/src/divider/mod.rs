// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod variant;

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};

use crate::styles::orientation::Orientation;
use crate::styles::text_align::TextAlign;
use crate::styles::CssClass;

// Re-export
pub use variant::Variant;
use zu_util::prop::attr_optional;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// Absolutely position the element.
    #[prop_or(false)]
    pub absolute: bool,

    #[prop_or_default]
    pub classes: String,

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
    let cls_list = vec![
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
            match props.orientation {
                Orientation::Vertical => "ZuDivider-withChildrenVertical",
                Orientation::Horizontal => "ZuDivider-withChildrenHorizontal",
            }
        },
        if props.orientation == Orientation::Horizontal {
            match props.text_align {
                TextAlign::Start => "ZuDivider-textAlignStart",
                TextAlign::End => "ZuDivider-textAlignEnd",
                _ => "",
            }
        } else {
            ""
        },
    ];

    let cls = classes!(cls_list);
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
    let style = attr_optional(&props.style);

    if props.children.is_empty() {
        html! {
            <@{component} class={cls} style={style} role={role} />
        }
    } else {
        html! {
            <@{component} class={cls} style={style} role={role}>
                <span class={wrapper_cls}>
                    {for props.children.iter()}
                </span>
            </@>
        }
    }
}
