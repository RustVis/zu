// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DirectionsOffSharp)]
pub fn directions_off_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DirectionsOffSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="m1.39 4.22 3.99 3.99L1.59 12l10.42 10.4 3.79-3.79 3.99 3.99 1.41-1.41L2.81 2.81 1.39 4.22zm8.6 8.6V15h-2v-4.18l2 2z"/>
        </SvgIcon>
    }
}
