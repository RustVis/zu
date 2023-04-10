// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use yew::{function_component, Html, Properties, html};

use crate::types::{ElementSize, RelativePosition};
use super::default_svg::default_svg;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct SpinnerProps {
    #[prop_or_default]
    pub label: Option<String>,

    /// Where the label is placed relative to the Spinner.
    #[prop_or_default]
    pub label_position: RelativePosition,

    #[prop_or_default]
    pub size: ElementSize,
}

#[function_component(Spinner)]
pub fn spinner(props: &SpinnerProps) -> Html {
    html! {
        <div>
            if props.label.is_some() &&
                (props.label_position == RelativePosition::Below ||
                    props.label_position == RelativePosition::After) {
                <label>{ props.label.clone() }</label>
            }
            <span>
                { default_svg() }
            </span>
            if props.label.is_some() &&
                (props.label_position == RelativePosition::Below ||
                    props.label_position == RelativePosition::After) {
                <label>{ props.label.clone() }</label>
            }
        </div>
    }
}