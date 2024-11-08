// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

mod variant;

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};
use zu_util::prop::ToAttr;

use crate::styles::CssClass;
pub use variant::Variant;

pub type Elevation = i32;
pub const ELEVATION_DEFAULT: Elevation = 1;
pub const ELEVATION_MIN: Elevation = 0;
pub const ELEVATION_MAX: Elevation = 24;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    /// Override or extend the styles applied to the component.
    #[prop_or_default]
    pub classes: Classes,

    /// Default value is `div`.
    #[prop_or(AttrValue::from("div"))]
    pub component: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or(ELEVATION_DEFAULT)]
    /// Shadow depth, corresponds to dp in the spec.
    ///
    /// It accepts values between 0 and 24 inclusive.
    pub elevation: Elevation,

    /// If true, rounded corners are disabled.
    #[prop_or(false)]
    pub square: bool,

    #[prop_or_default]
    /// The variant to use.
    pub variant: Variant,
}

#[function_component(Paper)]
pub fn paper(props: &Props) -> Html {
    if props.variant == Variant::Elevation
        && (props.elevation < ELEVATION_MIN || props.elevation > ELEVATION_MAX)
    {
        log::warn!("elevation out of range, expected {ELEVATION_MIN}-{ELEVATION_MAX}");
    };

    let root_cls = classes!(
        "ZuPaper-root",
        props.variant.css_class(),
        if props.square { "" } else { "ZuPaper-rounded" },
        if props.variant == Variant::Elevation {
            format!("ZuPaper-elevation-{}", props.elevation)
        } else {
            String::new()
        },
        props.classes.clone(),
    );

    html! {
        <@{props.component.to_string()}
            class={root_cls}
            style={props.style.to_attr()}>
            {for props.children.iter()}
        </@>
    }
}
