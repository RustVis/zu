// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MultipleStopSharp)]
pub fn multiple_stop_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MultipleStopSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,4l4,4l-4,4V9h-4V7h4V4z M7,17h4v-2H7v-3l-4,4l4,4V17z M19,15h-2v2h2V15z M15,15h-2v2h2V15z M11,7H9v2h2V7z M7,7H5v2h2 V7z"/>
        </SvgIcon>
    }
}
