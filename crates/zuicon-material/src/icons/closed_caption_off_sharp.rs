// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ClosedCaptionOffSharp)]
pub fn closed_caption_off_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ClosedCaptionOffSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,4H3v16h18V4z M11,11H9.5v-0.5h-2v3h2V13H11v2H6V9h5V11z M18,11h-1.5v-0.5h-2v3h2V13H18v2h-5V9h5V11z"/>
        </SvgIcon>
    }
}
