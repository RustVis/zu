// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::type_repetition_in_bounds)]

use std::fmt;
use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};

use crate::button_base::ButtonBase;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props<T: PartialEq> {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub label: Option<Html>,

    /// If true, the BottomNavigationAction will show its label.
    #[prop_or(false)]
    pub show_label: bool,

    #[prop_or(false)]
    pub selected: bool,

    #[prop_or_default]
    pub style: AttrValue,

    /// You can provide your own value. Otherwise, we fallback to the child position index.
    #[prop_or_default]
    pub value: Option<T>,
}

#[function_component(BottomNavigationAction)]
pub fn bottom_navigation_action<T>(props: &Props<T>) -> Html
where
    T: fmt::Debug + Clone + PartialEq,
{
    let root_cls = classes!(
        "ZuBottomNavigationAction-root",
        if !props.show_label && !props.selected {
            "ZuBottomNavigation-iconOnly"
        } else {
            ""
        },
        if props.label.is_none() {
            "ZuBottomNavigation-noLabel"
        } else {
            ""
        },
        if props.selected {
            "ZuBottomNavigation-selected"
        } else {
            ""
        },
    );
    let label_cls = classes!(
        "ZuBottomNavigationAction-label",
        if !props.show_label && !props.selected {
            "ZuBottomNavigation-iconOnly"
        } else {
            ""
        },
        if props.selected {
            "ZuBottomNavigation-selected"
        } else {
            ""
        },
    );

    // TODO(Shaohua): Handle onChange and onClick events

    html! {
        <ButtonBase classes={root_cls}
            style={&props.style}
            focus_ripple={true}>
            {props.icon.clone()}
            <span class={label_cls}>
                {props.label.clone()}
            </span>
        </ButtonBase>
    }
}
