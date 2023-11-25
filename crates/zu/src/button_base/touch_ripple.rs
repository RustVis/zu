// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::{classes, function_component, html, Classes, Html, Properties};

pub const DELAY_RIPPLE: i32 = 80;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    /// If `true`, the ripple starts at the center of the component
    /// rather than at the point of interaction.
    #[prop_or(false)]
    pub center: bool,

    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(TouchRipple)]
pub fn touch_ripple(props: &Props) -> Html {
    let root_cls = classes!("ZuTouchRipple-root", props.classes.clone(),);

    html! {
        <span class={root_cls}>
        </span>
    }
}
