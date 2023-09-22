// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{
    classes, function_component, html, AttrValue, Callback, Children, Classes, Event, Html,
    Properties,
};
use zu_util::prop::ToAttr;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    /// Default value is `div`.
    #[prop_or(AttrValue::from("div"))]
    pub component: AttrValue,

    /// Callback fired when the value changes.
    pub on_change: Option<Callback<(Event, i32), ()>>,

    /// If true, all BottomNavigationActions will show their labels.
    #[prop_or(false)]
    pub show_labels: bool,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(BottomNavigation)]
pub fn bottom_navigation(props: &Props) -> Html {
    let root_cls = classes!("ZuBottomNavigation", props.classes.clone());

    // TODO(Shaohua): Add on_change, value and show_label to children props.

    html! {
        <@{props.component.to_string()} class={root_cls}
            style={props.style.to_attr()}>
            {for props.children.iter()}
        </@>
    }
}
