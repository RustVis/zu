// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod size;

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::label_variant::LabelVariant;
use crate::styles::size::Size;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// The content of the component.
    #[prop_or_default]
    pub children: Children,

    /// Override or extend the styles applied to the component.
    #[prop_or_default]
    pub classes: Classes,

    /// The component used for the root node.
    ///
    /// Either a string to use a HTML element or a component.
    #[prop_or(AttrValue::from("p"))]
    pub component: AttrValue,

    /// If `true`, the helper text should be displayed in a disabled state.
    #[prop_or(false)]
    pub disabled: bool,

    /// If `true`, helper text should be displayed in an error state.
    #[prop_or(false)]
    pub error: bool,

    /// If `true`, the helper text should use filled classes key.
    #[prop_or(false)]
    pub filled: bool,

    /// If `true`, the helper text should use focused classes key.
    #[prop_or(false)]
    pub focused: bool,

    /// If `true`, will adjust vertical spacing. This is normally obtained via context from `FormControl`.
    #[prop_or(true)]
    pub dense_margin: bool,

    #[prop_or_default]
    pub id: AttrValue,

    /// If `true`, the helper text should use required classes key.
    #[prop_or(false)]
    pub required: bool,

    #[prop_or(Size::Medium)]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    /// The variant to use.
    #[prop_or_default]
    pub variant: LabelVariant,
}

#[function_component(FormHelperText)]
pub fn form_helper_text(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuFormHelperText-root",
        if props.disabled {
            "ZuFormHelperText-disabled"
        } else {
            ""
        },
        if props.error {
            "ZuFormHelperText-error"
        } else {
            ""
        },
        size::css_class(props.size),
        if props.variant.is_contained() {
            "ZuFormHelperText-contained"
        } else {
            ""
        },
        if props.focused {
            "ZuFormHelperText-focused"
        } else {
            ""
        },
        if props.filled {
            "ZuFormHelperText-filled"
        } else {
            ""
        },
        if props.required {
            "ZuFormHelperText-required"
        } else {
            ""
        },
        props.classes.clone()
    );

    html! {
        <@{props.component.to_string()}
            class={root_cls}
            id={&props.id}
        >
            {for props.children.iter()}
        </@>
    }
}
