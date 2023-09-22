// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WatchOffTwoTone)]
pub fn watch_off_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WatchOffTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.89,5.27L13.51,4h-3.02l-0.38,1.27C11.93,4.76,13.34,5.11,13.89,5.27z" opacity=".3"/><path d="M10.11,18.73L10.49,20h3.02l0.38-1.27C12.07,19.24,10.66,18.89,10.11,18.73z" opacity=".3"/><path d="M12,7c2.76,0,5,2.24,5,5c0,0.64-0.13,1.25-0.35,1.82l1.5,1.5C18.69,14.33,19,13.2,19,12c0-2.22-1.03-4.19-2.64-5.47L15,2 H9L8.04,5.21l2.14,2.14C10.75,7.13,11.36,7,12,7z M10.49,4h3.02l0.38,1.27c-0.55-0.16-1.97-0.51-3.78,0L10.49,4z"/><path d="M2.81,2.81L1.39,4.22l4.46,4.46C5.31,9.67,5,10.8,5,12c0,2.22,1.03,4.19,2.64,5.47L9,22h6l0.96-3.21l3.82,3.82l1.41-1.41 L2.81,2.81z M13.51,20h-3.02l-0.38-1.27c0.55,0.15,1.97,0.51,3.78,0L13.51,20z M12,17c-2.76,0-5-2.24-5-5 c0-0.64,0.13-1.25,0.35-1.82l6.47,6.47C13.25,16.87,12.64,17,12,17z"/>
        </SvgIcon>
    }
}
