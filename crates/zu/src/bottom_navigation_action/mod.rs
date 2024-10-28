// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::button_base::ButtonBase;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub label: Option<Html>,

    /// If true, the `BottomNavigationAction` will show its label.
    #[prop_or(false)]
    pub show_label: bool,

    #[prop_or(false)]
    pub selected: bool,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(BottomNavigationAction)]
pub fn bottom_navigation_action(props: &Props) -> Html {
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
        props.classes.clone(),
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

    // TODO(Shaohua): Check value of focus_ripple.
    html! {
        <ButtonBase
            classes={root_cls}
            style={&props.style}
            focus_ripple={true}>
            {props.icon.clone()}
            <span class={label_cls}>
                {props.label.clone()}
            </span>
        </ButtonBase>
    }
}
