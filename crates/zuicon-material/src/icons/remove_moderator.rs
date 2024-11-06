// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RemoveModerator)]
pub fn remove_moderator(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RemoveModerator"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0z" fill="none"/><path d="M22.27 21.73l-3.54-3.55L5.78 5.23 2.27 1.72 1 2.99 3.01 5H3v6c0 5.55 3.84 10.74 9 12 2.16-.53 4.08-1.76 5.6-3.41L21 23l1.27-1.27zM13 9.92l6.67 6.67C20.51 14.87 21 12.96 21 11V5l-9-4-5.48 2.44L11 7.92l2 2z"/>
        </SvgIcon>
    }
}
