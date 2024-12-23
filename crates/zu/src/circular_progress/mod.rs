// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

mod color;
mod size;
mod variant;

use std::f64::consts::PI;
use yew::{classes, function_component, html, AttrValue, Html, Properties};

use crate::styles::color::Color;
use crate::styles::CssClass;

// Re-export
pub use size::Size;
pub use variant::Variant;

const SIZE: i32 = 44;
const SIZE_STR: &str = "44";

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub color: Color,

    /// The size of the component. Default is 40px.
    #[prop_or(Size::Num(40))]
    pub size: Size,

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
    pub style: AttrValue,

    /// Show label or not.
    ///
    /// This only works if variant is determinate.
    #[prop_or(false)]
    pub with_label: bool,
}

#[function_component(CircularProgress)]
pub fn circular_progress(props: &Props) -> Html {
    let cls = classes!(
        "ZuCircularProgress-root",
        props.variant.css_class(),
        color::css_class(props.color),
    );

    let size_style = props.size.css_value();
    let svg_style = [props.style.as_str(), &props.color.text_color(), &size_style].join(";");

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

    // TODO(Shaohua): Convert type to Option<String>
    let label = if props.with_label {
        format!("{}%", props.value)
    } else {
        String::new()
    };

    let progress = html! {
        <span class={cls} style={svg_style}>
            <svg class="ZuCircularProgress-svg"
                viewBox={format!("{} {} {SIZE} {SIZE}", SIZE / 2, SIZE / 2)}>
                <circle class="ZuCircularProgress-circle"
                    style={circle_style}
                    cx={SIZE_STR}
                    cy={SIZE_STR}
                    r={radius.to_string()}
                    fill="none"
                    stroke-width={props.thickness.to_string()}></circle>
            </svg>
        </span>
    };

    if !props.with_label {
        return progress;
    }

    // TODO(Shaohua): Replace with <Caption>
    html! {
        <div class="ZuCircularProgress-labelContainer"
            style={size_style}>
            {progress}
            <span class="ZuCircularProgress-label" title={label.clone()}>{label}</span>
        </div>
    }
}
