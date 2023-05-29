// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod color;
mod size;
mod variant;

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};

use crate::button_base::ButtonBase;
use crate::styles::{color::Color, size::Size, CssClass};
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub color: Color,

    #[prop_or_default]
    pub component: AttrValue,

    /// If true, the component is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// If true, no elevation is used.
    #[prop_or(false)]
    pub disable_elevation: bool,

    /// If true, the keyboard focus ripple is disabled.
    #[prop_or(false)]
    pub disable_focus_ripple: bool,

    /// If true, the ripple effect is disabled.
    #[prop_or(false)]
    pub disable_ripple: bool,

    /// Element placed after the children.
    #[prop_or_default]
    pub end_icon: Option<Html>,

    /// If true, the button will take up the full width of its container.
    #[prop_or(false)]
    pub full_width: bool,

    /// The URL to link to when the button is clicked.
    ///
    ///  If defined, an a element will be used as the root node.
    #[prop_or_default]
    pub href: AttrValue,

    /// The size of the component.
    ///
    ///  small is equivalent to the dense button styling.
    #[prop_or_default]
    pub size: Size,

    /// Element placed before the children.
    #[prop_or_default]
    pub start_icon: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    // TODO(Shaohua): Add mouse event callbacks

    let root_cls = classes!(
        "ZuButton-root",
        props.classes.as_str().to_owned(),
        props.variant.css_class(),
        color::color_class(&props.color),
        size::css_class(&props.size),
        if props.disable_elevation {
            "ZuButton-disable_elevation"
        } else {
            ""
        },
        if props.full_width {
            "ZuButton-fullWidth"
        } else {
            ""
        }
    );

    //let label_cls = "ZuButton-label";
    //let start_icon_cls = classes!("ZuButton-startIcon", props.size.icon_class(),);
    //let end_icon_cls = classes!("ZuButton-endIcon", props.size.icon_class(),);

    // TODO(Shaohua): Set class for start_icon and end_icon.

    html! {
        <ButtonBase classes={root_cls}>
            if let Some(start_icon) = &props.start_icon {
                {start_icon.clone()}
            }
            {for props.children.iter()}
            if let Some(end_icon) = &props.end_icon {
                {end_icon.clone()}
            }
        </ButtonBase>
    }
}
