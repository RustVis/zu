// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::f64::consts::PI;
use yew::{classes, function_component, html, AttrValue, Html, Properties};

use crate::styles::color::Color;
use crate::styles::size::Size;
use crate::styles::CssClass;

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
    let mut class_list = vec!["ZuCircularProgress-root", props.variant.css_class()];
    match props.color {
        Color::Primary => class_list.push("ZuCircularProgress-colorPrimary"),
        Color::Secondary => class_list.push("ZuCircularProgress-colorSecondary"),
        _ => (),
    }
    let cls = classes!(class_list);

    let mut styles = vec![props.style.as_str().to_string(), props.color.text_color()];
    // TODO(Shaohua): Read from css.
    let size = match props.size {
        Size::XSmall => 8,
        Size::Small => 12,
        Size::Middle => 14,
        Size::Large => 18,
        Size::XLarge => 24,
        Size::Num(num) => num,
    };
    let size_style = format!("width: {size}px; height: {size}px");
    styles.push(size_style.clone());

    let style = styles.join(";");

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

    let progress = html! {
        <span class={cls} style={style}>
            <svg class="ZuCircularProgress-svg"
                viewBox={format!("{} {} {SIZE} {SIZE}", SIZE / 2, SIZE / 2)}>
                <circle class="ZuCircularProgress-circle"
                    style={circle_style}
                    cx={SIZE.to_string()}
                    cy={SIZE.to_string()}
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
