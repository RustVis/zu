// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SettingsInputSvideoRounded)]
pub fn settings_input_svideo_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SettingsInputSvideoRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,20c-4.42,0-8-3.58-8-8s3.58-8,8-8s8,3.58,8,8 S16.42,20,12,20z"/><path d="M15,7.5C15,6.67,14.33,6,13.5,6h-3C9.67,6,9,6.67,9,7.5S9.67,9,10.5,9h3C14.33,9,15,8.33,15,7.5z"/>
        </SvgIcon>
    }
}
