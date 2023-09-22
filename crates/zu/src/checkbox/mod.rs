// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod color;

use yew::{classes, function_component, html, AttrValue, Callback, Classes, Html, Properties};
use zu_util::prop::ToAttr;

use crate::styles::color::Color;
use crate::styles::size::Size;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub checked: bool,

    /// The icon to display when the component is checked.
    ///
    /// Default is CheckBox icon.
    #[prop_or_default]
    pub checked_icon: Option<Html>,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub color: Color,

    /// The default checked state. Use when the component is not controlled.
    #[prop_or(false)]
    pub default_checked: bool,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub disable_ripple: bool,

    /// The icon to display when the component is unchecked.
    ///
    /// Default is CheckBoxOutlineBlank
    #[prop_or_default]
    pub icon: Option<Html>,

    /// The id of the input element.
    #[prop_or_default]
    pub id: AttrValue,

    /// If true, the component appears indeterminate.
    #[prop_or(false)]
    pub indeterminate: bool,

    /// Default is IndeterminateCheckBox
    #[prop_or_default]
    pub indeterminate_icon: Option<Html>,

    /// Callback fired when the state is changed.
    #[prop_or_default]
    pub on_change: Option<Callback<()>>,

    /// If true, the input element is required.
    #[prop_or(false)]
    pub required: bool,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,
    //pub value: T,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuCheckbox-root",
        if props.indeterminate {
            "ZuCheckbox-indeterminate"
        } else {
            ""
        },
        color::css_class(props.color),
        if props.disable_ripple {
            ""
        } else {
            "ZuCheckbox-enableRipple"
        },
        props.classes.clone(),
    );

    // TODO(Shaohua): Add SwitchBase.
    // TODO(Shaohua): Add default icons.
    html! {
       <div class={root_cls} style={props.style.to_attr()}>
        </div>
    }
}
