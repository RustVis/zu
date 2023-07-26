// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::type_repetition_in_bounds)]

use std::rc::Rc;
use yew::{
    classes, function_component, html, AttrValue, Callback, ChildrenWithProps, Classes, Html,
    MouseEvent, Properties,
};

use crate::styles::color::Color;
use crate::styles::orientation::Orientation;
use crate::styles::size::Size;
use crate::toggle_button::{Props as ToggleButtonProps, ToggleButton};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props<T>
where
    T: Clone + PartialEq + 'static,
{
    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub children: ChildrenWithProps<ToggleButton<T>>,

    #[prop_or_default]
    pub classes: Classes,

    // FIXME(Shaohua): No Standard in Color.
    #[prop_or_default]
    pub color: Color,

    #[prop_or(false)]
    pub disabled: bool,

    /// If `true`, only allow one of the child ToggleButton values to be selected.
    #[prop_or(false)]
    pub exclusive: bool,

    #[prop_or(false)]
    pub full_width: bool,

    /// Callback fired when the value changes.
    #[prop_or_default]
    pub on_change: Option<Callback<MouseEvent>>,

    #[prop_or(Orientation::Horizontal)]
    pub orientation: Orientation,

    #[prop_or(Size::Medium)]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    /// This value is used if `exclusive` = true.
    #[prop_or_default]
    pub value: Option<T>,

    /// This value is used if `exclusive` = false.
    #[prop_or_default]
    pub values: Vec<T>,
}

#[function_component(ToggleButtonGroup)]
pub fn toggle_button_group<T>(props: &Props<T>) -> Html
where
    T: Clone + PartialEq + std::fmt::Debug + 'static,
{
    let root_cls = classes!(
        "ZuToggleButtonGroup-root",
        if props.orientation == Orientation::Vertical {
            "ZuToggleButtonGroup-vertical"
        } else {
            ""
        },
        if props.full_width {
            "ZuToggleButtonGroup-fullWidth"
        } else {
            ""
        },
        props.classes.clone(),
    );

    // TODO(Shaohua): Handle on change event.

    let children: Vec<_> = props
        .children
        .iter()
        .map(|mut item| {
            let p = Rc::<ToggleButtonProps<T>>::make_mut(&mut item.props);
            p.color = props.color;
            // TODO(Shaohua): pass onChange and selected props.
            p.disabled = props.disabled;
            p.full_width = props.full_width;
            p.size = props.size;
            item
        })
        .collect();
    let children = ChildrenWithProps::new(children);

    html! {
        <div role="group" class={root_cls}>
            {for children.iter()}
        </div>
    }
}
