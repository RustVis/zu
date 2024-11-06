// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VolumeMuteRounded)]
pub fn volume_mute_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VolumeMuteRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7 10v4c0 .55.45 1 1 1h3l3.29 3.29c.63.63 1.71.18 1.71-.71V6.41c0-.89-1.08-1.34-1.71-.71L11 9H8c-.55 0-1 .45-1 1z"/>
        </SvgIcon>
    }
}
