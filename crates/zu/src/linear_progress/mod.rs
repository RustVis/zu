// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::color::ColorVariant;
use crate::styles::CssClass;
use yew::{function_component, html, Html, Properties};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Buffer,
    Determinate,
    Indeterminate,
    Query,
}

impl Default for Variant {
    fn default() -> Self {
        Self::Indeterminate
    }
}

impl CssClass for Variant {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Buffer => "ZuLinearProgress-buffer",
            Self::Determinate => "ZuLinearProgress-determinate",
            Self::Indeterminate => "ZuLinearProgress-indeterminate",
            Self::Query => "ZuLinearProgress-query",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub color: ColorVariant,

    /// The value of the progress indicator for the determinate variant.
    ///
    /// Value between 0 and 100.
    #[prop_or(0)]
    pub value: i32,

    /// The value for the buffer variant. Value between 0 and 100.
    #[prop_or(0)]
    pub value_buffer: i32,

    /// Use indeterminate when there is no progress value.
    #[prop_or_default]
    pub variant: Variant,

    /// Override root style.
    #[prop_or_default]
    pub style: String,
}

#[function_component(LinearProgress)]
pub fn linear_progress(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
