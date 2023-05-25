// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SubtitlesOffSharp)]
pub fn subtitles_off_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SubtitlesOffSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M1.04,3.87L2,4.83V20h15.17l2.96,2.96l1.41-1.41L2.45,2.45L1.04,3.87z M4,12h4v2H4V12z M4,16h9.17L14,16.83V18H4V16z"/>
        </SvgIcon>
    }
}
