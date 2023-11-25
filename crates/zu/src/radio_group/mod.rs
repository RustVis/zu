// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Callback, Children, Classes, Html, Properties};

use crate::form_group::FormGroup;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub aria_labelled_by: AttrValue,

    /// The content of the component.
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    /// The default value.
    ///
    /// Use when the component is not controlled.
    #[prop_or_default]
    pub default_value: AttrValue,

    /// The name used to reference the value of the control.
    ///
    /// If you don't provide this prop, it falls back to a randomly generated name.
    #[prop_or_default]
    pub name: AttrValue,

    /// Callback fired when a radio button is selected.
    #[prop_or_default]
    pub on_change: Option<Callback<String>>,

    /// Show in row mode or not.
    ///
    /// Default is false, which means showing in column mode.
    #[prop_or(false)]
    pub row: bool,

    #[prop_or_default]
    pub style: AttrValue,

    /// Value of the selected radio button. The DOM API casts this to a string.
    #[prop_or_default]
    pub value: AttrValue,
}

#[function_component(RadioGroup)]
pub fn radio_group(props: &Props) -> Html {
    html! {
        <FormGroup
            classes={&props.classes}
            role="radio-group"
            style={&props.style}
        >
            {for props.children.iter()}
        </FormGroup>
    }
}
