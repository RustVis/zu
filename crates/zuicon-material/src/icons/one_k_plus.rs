// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OneKPlus)]
pub fn one_k_plus(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OneKPlus"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 15H7.5v-4.5H6V9h3v6zm4.75 0L12 12.75V15h-1.5V9H12v2.25L13.75 9h1.75l-2.25 3 2.25 3h-1.75zm5.75-2.5H18V14h-1v-1.5h-1.5v-1H17V10h1v1.5h1.5v1z"/>
        </SvgIcon>
    }
}
