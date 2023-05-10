// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::prelude::*;

use crate::styles::color::ColorVariant;
use crate::styles::size::SizeVariant;
use crate::styles::{CssClass, CssValue};

const ROOT_CLS: &str = "ZuCircularProgress-root";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Determinate,
    Indeterminate,
}

impl Default for Variant {
    fn default() -> Self {
        Self::Indeterminate
    }
}

impl CssClass for Variant {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Determinate => "ZuCircularProgress-determinate",
            Self::Indeterminate => "ZuCircularProgress-indeterminate",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub color: ColorVariant,

    /// The size of the component. Default is 40px.
    #[prop_or(SizeVariant::Num(40))]
    pub size: SizeVariant,

    /// The sickness of the circle.
    #[prop_or(3.6)]
    pub sickness: f64,

    /// The value of the progress indicator for the determinate variant.
    ///
    /// Value between 0 and 100.
    #[prop_or(0)]
    pub value: i32,

    /// Use indeterminate when there is no progress value.
    #[prop_or_default]
    pub variant: Variant,

    /// If true, the shrink animation is disabled. This only works if variant is indeterminate.
    #[prop_or(false)]
    pub disable_shrink: bool,

    #[prop_or_default]
    pub style: String,
}

#[function_component(CircularProgress)]
pub fn circular_progress(props: &Props) -> Html {
    let mut class_list = vec![ROOT_CLS, props.variant.css_class()];
    match props.color {
        ColorVariant::Primary => class_list.push("ZuCircularProgress-colorPrimary"),
        ColorVariant::Secondary => class_list.push("MuiCircularProgress-colorSecondary"),
        _ => (),
    }
    let cls = classes!(class_list);

    let mut styles = vec![props.style.clone(), props.color.css_value()];
    let size = props.size.css_value();
    styles.push(format!("width: {size}; height: {size}"));

    // if props.variant == Variant::Determinate && props.value >= 0 {
    //     styles.push(format!("stroke-dash-array"));
    // }
    let style = styles.join(";");

    html! {
        <span class={ cls } style={ style }>
            <svg class="ZuCircularProgress-svg" viewBox="22 22 44 44">
                <circle class="ZuCircularProgress-circle"
                    cx={ 44 } cy={ 44 } r={ 20.2 }
                    stroke-width={ props.sickness.to_string() }></circle>
            </svg>
        </span>
    }
}
