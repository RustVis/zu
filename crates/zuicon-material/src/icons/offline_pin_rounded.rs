// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OfflinePinRounded)]
pub fn offline_pin_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OfflinePinRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0z" fill="none"/><path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zm4 16H8c-.55 0-1-.45-1-1s.45-1 1-1h8c.55 0 1 .45 1 1s-.45 1-1 1zm-6.41-4.71L7.7 11.4c-.39-.39-.39-1.01 0-1.4.39-.39 1.01-.39 1.4 0l1.2 1.2 4.6-4.6c.39-.39 1.01-.39 1.4 0 .39.39.39 1.01 0 1.4l-5.29 5.29c-.39.39-1.03.39-1.42 0z"/>
        </SvgIcon>
    }
}
