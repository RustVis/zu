// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HiveTwoTone)]
pub fn hive_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HiveTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.5,9l-2.25-4h-3.31l-1.69-3h-4.5L8.06,5H4.75L2.5,9l1.69,3L2.5,15l2.25,4h3.31l1.69,3h4.5l1.69-3h3.31l2.25-4l-1.69-3 L21.5,9z M8.06,17H5.92l-1.12-2l1.12-2h2.14l1.12,2L8.06,17z M8.06,11H5.92L4.79,9l1.12-2h2.14l1.12,2L8.06,11z M13.08,20h-2.16 L9.8,18.02L10.94,16h2.12l1.13,2.02L13.08,20z M9.81,12l1.12-2h2.12l1.12,2l-1.12,2h-2.12L9.81,12z M13.06,8h-2.12L9.8,5.98 L10.92,4h2.16l1.12,1.98L13.06,8z M18.08,17h-2.14l-1.12-2l1.12-2h2.14l1.12,2L18.08,17z M18.08,11h-2.14l-1.12-2l1.12-2h2.14 l1.12,2L18.08,11z"/>
        </SvgIcon>
    }
}
