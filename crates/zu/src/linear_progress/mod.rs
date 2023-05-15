// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod color;
mod variant;

use yew::{classes, function_component, html, Html, Properties};

use crate::styles::color::Color;
use crate::styles::CssClass;

// Re-export
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub color: Color,

    /// Override root style.
    #[prop_or_default]
    pub style: String,

    /// The value of the progress indicator for the determinate variant.
    ///
    /// Value between 0 and 100.
    #[prop_or(0)]
    pub value: i32,

    /// The value for the buffer variant. Value between 0 and 100.
    ///
    /// Default is None.
    #[prop_or_default]
    pub value_buffer: Option<i32>,

    /// Use indeterminate when there is no progress value.
    #[prop_or_default]
    pub variant: Variant,

    /// Show label text or not.
    #[prop_or(false)]
    pub with_label: bool,
}

#[function_component(LinearProgress)]
pub fn linear_progress(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuLinearProgress-root",
        color::root_class(&props.color),
        props.variant.css_class()
    );
    let _dashed_cls = classes!("ZuLinearProgress-dashed", color::dashed_class(&props.color),);
    let _bar1_cls = classes!(
        "ZuLinearProgress-bar",
        color::bar_class(&props.color),
        match props.variant {
            Variant::Indeterminate | Variant::Query => "ZuLinearProgress-bar1Indeterminate",
            Variant::Determinate => "ZuLinearProgress-bar1Determinate",
            Variant::Buffer => "ZuLinearProgress-bar1Buffer",
        },
    );
    let _bar2_cls = classes!(
        "ZuLinearProgress-bar",
        if props.variant == Variant::Buffer {
            color::root_class(&props.color)
        } else {
            color::bar_class(&props.color)
        },
        match props.variant {
            Variant::Indeterminate | Variant::Query => "ZuLinearProgress-bar2Indeterminate",
            Variant::Buffer => "ZuLinearProgress-bar2Buffer",
            Variant::Determinate => "",
        }
    );

    html! {
        <div class={root_cls}>
        </div>
    }
}
