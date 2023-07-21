// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::paper::{Paper, Variant};

// TODO(Shaohua): Add Paper.Props part
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    /// If true, the card will use raised styling.
    #[prop_or(false)]
    pub raised: bool,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or(false)]
    pub square: bool,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    let elevation = if props.raised { 8 } else { 0 };

    html! {
        <Paper classes={props.classes.clone()}
            style={&props.style}
            square={props.square}
            variant={props.variant}
            elevation={elevation}>
            {for props.children.iter()}
        </Paper>
    }
}
