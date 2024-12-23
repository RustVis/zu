// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};

pub use crate::button_base::ButtonBase;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(CardActionArea)]
pub fn card_action_area(props: &Props) -> Html {
    let root_cls = classes!("ZuCardActionArea-root", props.classes.clone());

    html! {
        <ButtonBase classes={root_cls} style={&props.style}>
            {for props.children.iter()}
            <span class="ZuCardActionArea-focusHighlight"></span>
        </ButtonBase>
    }
}
