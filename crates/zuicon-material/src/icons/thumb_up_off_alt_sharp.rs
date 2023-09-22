// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ThumbUpOffAltSharp)]
pub fn thumb_up_off_alt_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ThumbUpOffAltSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14.17,1L7,8.18V21h12.31L23,12.4V8h-8.31l1.12-5.38L14.17,1z M1,9h4v12H1V9z"/>
        </SvgIcon>
    }
}
