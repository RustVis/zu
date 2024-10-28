// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

mod color;
mod size;

use yew::{classes, function_component, html, AttrValue, Callback, Classes, Html, Properties};

use crate::styles::color::Color;
use crate::styles::input_type::InputType;
use crate::styles::size::Size;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub aria_described_by: AttrValue,

    /// This prop helps users to fill forms faster.
    ///
    /// The name can be confusing, as it's more like an autofill.
    #[prop_or_default]
    pub auto_complete: AttrValue,

    /// If `true`, the `input` element is focused during the first mount.
    #[prop_or(false)]
    pub auto_focus: bool,

    /// Override or extend the styles applied to the component.
    #[prop_or_default]
    pub classes: Classes,

    /// The color of the component.
    #[prop_or_default]
    pub color: Color,

    // TODO(Shaohua): Add components
    /// The default value. Use when the component is not controlled.
    #[prop_or_default]
    pub default_value: AttrValue,

    /// If `true`, the component is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// If `true`, `GlobalStyles` for the auto-fill keyframes will not be injected/removed on mount/unmount.
    #[prop_or(false)]
    pub disable_injecting_global_styles: bool,

    /// End `InputAdornment` for this component.
    #[prop_or_default]
    pub end_adornment: Option<Html>,

    /// If `true`, the `input` will indicate an error.
    #[prop_or(false)]
    pub error: bool,

    /// If `true`, the `input` will take up the full width of its container.
    #[prop_or(false)]
    pub full_width: bool,

    /// The id of the `input` element.
    #[prop_or_default]
    pub id: AttrValue,
    // TODO(Shaohua): Add input component
    /// If `true`, will adjust vertical spacing.
    #[prop_or(true)]
    pub dense_margin: bool,

    #[prop_or_default]
    pub hidden_label: bool,

    /// Maximum number of rows to display when multiline option is set to true.
    #[prop_or_default]
    pub max_rows: Option<i32>,

    /// Minimum number of rows to display when multiline option is set to true.
    #[prop_or_default]
    pub min_rows: Option<i32>,

    /// If `true`, a `TextareaAutosize` element is rendered.
    #[prop_or(false)]
    pub multiline: bool,

    /// Name attribute of the `input` element.
    #[prop_or_default]
    pub name: AttrValue,

    #[prop_or_default]
    pub on_blur: Option<Callback<()>>,

    /// Callback fired when the value is changed.
    #[prop_or_default]
    pub on_change: Option<Callback<AttrValue>>,

    #[prop_or_default]
    pub on_focus: Option<Callback<()>>,

    /// The short hint displayed in the `input` before the user enters a value.
    #[prop_or_default]
    pub placeholder: AttrValue,

    /// It prevents the user from changing the value of the field.
    #[prop_or(false)]
    pub read_only: bool,

    /// If `true`, the `input` element is required.
    #[prop_or(false)]
    pub required: bool,

    /// Number of rows to display when multiline option is set to true.
    pub rows: Option<i32>,

    /// The size of the component.
    #[prop_or(Size::Medium)]
    pub size: Size,

    /// Start `InputAdornment` for this component.
    pub start_adornment: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,

    /// Type of the `input` element.
    #[prop_or_default]
    pub input_type: InputType,

    /// The value of the `input` element, required for a controlled component.
    #[prop_or_default]
    pub value: AttrValue,
}

#[function_component(InputBase)]
pub fn input_base(props: &Props) -> Html {
    let form_control = false;
    let focused = false;

    let root_cls = classes!(
        "ZuInputBase-root",
        color::css_class(props.color),
        if props.disabled {
            "ZuInputBase-disabled"
        } else {
            ""
        },
        if props.error { "ZuInputBase-error" } else { "" },
        if props.full_width {
            "ZuInputBase-fullWidth"
        } else {
            ""
        },
        if focused { "ZuInputBase-focused" } else { "" },
        if form_control {
            "ZuInputBase-formControl"
        } else {
            ""
        },
        size::css_class(props.size),
        if props.multiline {
            "ZuInputBase-multiline"
        } else {
            ""
        },
        if props.start_adornment.is_some() {
            "ZuInputBase-adornedStart"
        } else {
            ""
        },
        if props.end_adornment.is_some() {
            "ZuInputBase-adornedEnd"
        } else {
            ""
        },
        if props.hidden_label {
            "ZuInputBase-hiddenLabel"
        } else {
            ""
        },
        if props.read_only {
            "ZuInputBase-readOnly"
        } else {
            ""
        }
    );

    let input_cls = classes!(
        "ZuInputBase-input",
        if props.disabled {
            "ZuInputBase-disabled"
        } else {
            ""
        },
        if props.input_type == InputType::Search {
            "ZuInputBase-inputTypeSearch"
        } else {
            ""
        },
        if props.multiline {
            "ZuInputBase-inputMultiline"
        } else {
            ""
        },
        size::input_css_class(props.size),
        if props.hidden_label {
            "ZuInputBase-inputHiddenLabel"
        } else {
            ""
        },
        if props.start_adornment.is_some() {
            "ZuInputBase-inputAdornedStart"
        } else {
            ""
        },
        if props.end_adornment.is_some() {
            "ZuInputBase-inputAdornedEnd"
        } else {
            ""
        },
        if props.read_only {
            "ZuInputBase-readOnly"
        } else {
            ""
        },
    );

    html! {
        <div
            class={root_cls}>
            <input class={input_cls} />
        </div>
    }
}
