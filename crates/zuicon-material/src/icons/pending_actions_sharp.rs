// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PendingActionsSharp)]
pub fn pending_actions_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PendingActionsSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,12c-2.76,0-5,2.24-5,5s2.24,5,5,5c2.76,0,5-2.24,5-5S19.76,12,17,12z M18.65,19.35l-2.15-2.15V14h1v2.79l1.85,1.85 L18.65,19.35z M20,3h-5.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H4v19h8.11c-0.59-0.57-1.07-1.25-1.42-2H6V5h2v3h8V5h2v5.08 c0.71,0.1,1.38,0.31,2,0.6V3z M12,5c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1C13,4.55,12.55,5,12,5z"/>
        </SvgIcon>
    }
}
