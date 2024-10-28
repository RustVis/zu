// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{classes, function_component, html, Classes, Html, Properties};

use crate::internal::svg_icons::{RadioButtonChecked, RadioButtonUnchecked};
use crate::svg_icon::FontSize;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    /// If `true`, the component is checked.
    #[prop_or(false)]
    pub checked: bool,

    /// Override or extend the styles applied to the component.
    #[prop_or_default]
    pub classes: Classes,

    /// The size of the component.
    #[prop_or_default]
    pub font_size: FontSize,
}

#[function_component(RadioButtonIcon)]
pub fn radio_button_icon(props: &Props) -> Html {
    let dot_cls = classes!(
        "ZuRadioButtonIcon-dot",
        if props.checked {
            "ZuRadioButtonIcon-dotChecked"
        } else {
            ""
        }
    );
    html! {
        <span class="ZuRadioButtonIcon-root">
            <RadioButtonChecked
                classes={dot_cls}
                font_size={props.font_size} />
            <RadioButtonUnchecked
                classes="ZuRadioButtonIcon-background"
                font_size={props.font_size} />
        </span>
    }
}
