// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

//! The `TextField` is a convenience wrapper for the most common cases (80%).
//! It cannot be all things to all people, otherwise the API would grow out of control.
//!
//! It's important to understand that the text field is a simple abstraction:
//! - `FormControl`
//! - `InputLabel`
//! - `FilledInput`
//! - `OutlinedInput`
//! - `Input`
//! - `FormHelperText`

use yew::{
    classes, function_component, html, AttrValue, Callback, Children, Classes, Html, Properties,
};

use crate::filled_input::FilledInput;
use crate::form_control::FormControl;
use crate::form_helper_text::FormHelperText;
use crate::input::Input;
use crate::input_label::InputLabel;
use crate::outlined_input::OutlinedInput;
use crate::select::Select;
use crate::styles::color::Color;
use crate::styles::input_type::InputType;
use crate::styles::label_variant::LabelVariant;
use crate::styles::margin::Margin;
use crate::styles::size::Size;
use crate::utils::global_id;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// This prop helps users to fill forms faster.
    ///
    /// The name can be confusing, as it's more like an autofill.
    #[prop_or_default]
    pub auto_complete: AttrValue,

    /// If `true`, the `input` element is focused during the first mount.
    #[prop_or(false)]
    pub auto_focus: bool,

    #[prop_or_default]
    pub children: Children,

    /// Override or extend the styles applied to the component.
    #[prop_or_default]
    pub classes: Classes,

    /// The color of the component.
    #[prop_or_default]
    pub color: Color,

    /// The default value.
    ///
    /// Use when the component is not controlled.
    #[prop_or_default]
    pub default_value: AttrValue,

    /// If `true`, the component is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// If `true`, the label is displayed in an error state.
    #[prop_or(false)]
    pub error: bool,
    // TODO(Shaohua): Add form helper props.
    /// If `true`, the input will take up the full width of its container.
    #[prop_or(false)]
    pub full_width: bool,

    /// The helper text content.
    #[prop_or_default]
    pub helper_text: Option<Html>,

    /// The id of the `input` element.
    #[prop_or_default]
    pub id: AttrValue,

    // TODO(Shaohua): Add InputLabelProps
    // TODO(Shaohua): Add input props
    /// The label content.
    #[prop_or_default]
    pub label: Option<Html>,

    /// If `dense` or `normal`, will adjust vertical spacing of this and contained components.
    #[prop_or_default]
    pub margin: Margin,

    /// Maximum number of rows to display when multiline option is set to true.
    #[prop_or_default]
    pub max_rows: Option<i32>,

    /// Minimum number of rows to display when multiline option is set to true.
    #[prop_or_default]
    pub min_rows: Option<i32>,

    /// If `true`, a `textarea` element is rendered instead of an input.
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

    /// If `true`, the label is displayed as required and the `input` element is required.
    #[prop_or(false)]
    pub required: bool,

    /// Number of rows to display when multiline option is set to true.
    #[prop_or_default]
    pub rows: Option<i32>,

    /// ender a `Select` element while passing the Input element to `Select` as `input` parameter.
    ///
    /// If this option is set you must pass the options of the select as children.
    #[prop_or(false)]
    pub select: bool,
    // TODO(Shaohua): Add select props
    /// The size of the component.
    #[prop_or(Size::Medium)]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    /// Type of the `input` element.
    #[prop_or_default]
    pub input_type: InputType,

    #[prop_or(false)]
    pub input_read_only: bool,

    #[prop_or(false)]
    pub input_label_shrink: bool,

    /// The value of the `input` element, required for a controlled component.
    #[prop_or_default]
    pub value: AttrValue,

    /// The variant to use.
    #[prop_or_default]
    pub variant: LabelVariant,
}

#[function_component(TextField)]
pub fn text_field(props: &Props) -> Html {
    // TODO(Shaohua): Support Select
    // TODO(Shaohua): Check outlined style.

    let id = AttrValue::from(global_id());
    let input_label_id = AttrValue::from(format!("{id}-label"));
    let helper_text_id = AttrValue::from(format!("{id}-helper-text"));
    let root_cls = classes!("ZuTextField-root", props.classes.clone());

    let input_variant = get_input_variant(props, &id, &helper_text_id);

    let input_content = if props.select {
        html! {
            <Select
                aria_described_by={&helper_text_id}
                id={&id}
                label_id={&input_label_id}
                input={input_variant}
                value={&props.value}
            >
                {for props.children.iter()}
            </Select>
        }
    } else {
        input_variant
    };

    html! {
        <FormControl
            classes={root_cls}
            color={props.color}
            disabled={props.disabled}
            error={props.error}
            full_width={props.full_width}
            required={props.required}
            variant={props.variant}
        >

        if let Some(label) = &props.label {
            <InputLabel
                id={input_label_id}
                html_for={id}
                shrink={props.input_label_shrink}>
                {label.clone()}
            </InputLabel>
        }

        {input_content}

        if let Some(helper_text) = &props.helper_text {
            <FormHelperText id={helper_text_id}>
                {helper_text.clone()}
            </FormHelperText>
        }

        </FormControl>
    }
}

fn get_input_variant(props: &Props, id: &AttrValue, helper_text_id: &AttrValue) -> Html {
    match props.variant {
        LabelVariant::Filled => html! {
            <Input
                aria_described_by={helper_text_id}
                auto_complete={&props.auto_complete}
                auto_focus={props.auto_focus}
                default_value={&props.default_value}
                full_width={props.full_width}
                multiline={props.multiline}
                name={&props.name}
                rows={props.rows}
                max_rows={props.max_rows}
                min_rows={props.min_rows}
                input_type={props.input_type}
                value={&props.value}
                id={id}
                on_blur={&props.on_blur}
                on_change={&props.on_change}
                on_focus={&props.on_focus}
                placeholder={&props.placeholder}
            />
        },
        LabelVariant::Outlined => html! {
            <FilledInput
                aria_described_by={helper_text_id}
                auto_complete={&props.auto_complete}
                auto_focus={props.auto_focus}
                default_value={&props.default_value}
                full_width={props.full_width}
                multiline={props.multiline}
                name={&props.name}
                rows={props.rows}
                max_rows={props.max_rows}
                min_rows={props.min_rows}
                input_type={props.input_type}
                value={&props.value}
                id={id}
                on_blur={&props.on_blur}
                on_change={&props.on_change}
                on_focus={&props.on_focus}
                placeholder={&props.placeholder}
            />
        },
        LabelVariant::Standard => html! {
            <OutlinedInput
                aria_described_by={helper_text_id}
                auto_complete={&props.auto_complete}
                auto_focus={props.auto_focus}
                default_value={&props.default_value}
                full_width={props.full_width}
                multiline={props.multiline}
                name={&props.name}
                rows={props.rows}
                max_rows={props.max_rows}
                min_rows={props.min_rows}
                input_type={props.input_type}
                value={&props.value}
                id={id}
                on_blur={&props.on_blur}
                on_change={&props.on_change}
                on_focus={&props.on_focus}
                placeholder={&props.placeholder}
            />
        },
    }
}
