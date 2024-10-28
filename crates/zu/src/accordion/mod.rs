// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{
    classes, function_component, html, AttrValue, Callback, Children, Classes, Event, Html,
    Properties,
};

use crate::paper::{Elevation, Paper, Variant, ELEVATION_DEFAULT};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    /// If true, expands the accordion by default.
    #[prop_or(false)]
    pub default_expanded: bool,

    #[prop_or(false)]
    pub disabled: bool,

    /// If true, it removes the margin between two expanded accordion items and the increase of height.
    #[prop_or(false)]
    pub disable_gutters: bool,

    /// If true, expands the accordion, otherwise collapse it.
    pub expanded: bool,

    pub on_change: Callback<(Event, bool), ()>,

    /// If true, rounded corners are disabled.
    #[prop_or(false)]
    pub square: bool,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or(ELEVATION_DEFAULT)]
    /// Shadow depth, corresponds to dp in the spec. It accepts values between 0 and 24 inclusive.
    pub elevation: Elevation,

    #[prop_or_default]
    /// The variant to use.
    pub variant: Variant,
}

#[function_component(Accordion)]
pub fn accordion(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuAccordion-root",
        if props.square {
            ""
        } else {
            "ZuAccordion-rounded"
        },
        if props.expanded {
            "ZuAccordion-expanded"
        } else {
            ""
        },
        if props.disabled {
            "ZuAccordion-disabled"
        } else {
            ""
        },
        if props.disable_gutters {
            ""
        } else {
            "ZuAccordion-gutters"
        },
        props.classes.clone(),
    );
    let region_cls = "ZuAccordion-region";

    html! {
        <Paper classes={root_cls}
            style={&props.style}
            square={props.square}>
            <div class={region_cls} role="region">
                {for props.children.iter()}
            </div>
        </Paper>
    }
}
