// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::f64::consts::PI;
use yew::prelude::*;

use crate::styles::color::ColorVariant;
use crate::styles::size::SizeVariant;
use crate::styles::{CssClass, CssValue};

const ROOT_CLS: &str = "ZuCircularProgress-root";
const SIZE: i32 = 44;

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

    /// The thickness of the circle.
    #[prop_or(3.6)]
    pub thickness: f64,

    /// The value of the progress indicator for the determinate variant.
    ///
    /// Value between 0 and 100.
    #[prop_or(0)]
    pub value: i32,

    /// Use indeterminate when there is no progress value.
    #[prop_or_default]
    pub variant: Variant,

    /// If true, the shrink animation is disabled.
    ///
    /// This only works if variant is indeterminate.
    #[prop_or(false)]
    pub disable_shrink: bool,

    /// Override root style.
    #[prop_or_default]
    pub style: String,

    /// Show label or not.
    ///
    /// This only works if variant is determinate.
    #[prop_or(false)]
    pub with_label: bool,
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

    let mut styles = vec![
        props.style.clone(),
        format!("color: {}", props.color.css_value()),
    ];
    // TODO(Shaohua): Read from css.
    let size = match props.size {
        SizeVariant::Tiny => 8,
        SizeVariant::Small => 12,
        SizeVariant::Regular => 14,
        SizeVariant::Large => 18,
        SizeVariant::XLarge => 24,
        SizeVariant::Num(num) => num,
    };
    styles.push(format!("width: {size}px; height: {size}px"));

    let style = styles.join(";");
    log::info!("style: {style}");

    let radius: f64 = (f64::from(SIZE) - props.thickness) / 2.0;
    let mut circle_styles = vec![];
    if props.variant == Variant::Determinate && props.value >= 0 {
        let circumference = 2.0 * radius * PI;
        let stroke_dasharray = circumference;
        let stroke_dashoffset = (f64::from(100 - props.value) / 100.0) * circumference;

        circle_styles.push(format!("stroke-dasharray: {stroke_dasharray}px",));
        circle_styles.push(format!("stroke-dashoffset: {stroke_dashoffset}px",));
    };
    let circle_style = circle_styles.join(";");

    let label = if props.with_label {
        format!("{}%", props.value)
    } else {
        String::new()
    };

    html! {
        <div class={ cls } style={ style }>
            <svg class="ZuCircularProgress-svg"
                viewBox={ format!("{} {} {SIZE} {SIZE}", SIZE / 2, SIZE / 2) } >
                <circle class="ZuCircularProgress-circle"
                    style={ circle_style }
                    cx={ SIZE.to_string() }
                    cy={ SIZE.to_string() }
                    r={ radius.to_string() }
                    fill="none"
                    stroke-width={ props.thickness.to_string()}></circle>
            </svg>

            if props.with_label {
                <span class="ZuCircularProgress-label" title={ label.clone() }>{ &label }</span>
            }
        </div>
    }
}
