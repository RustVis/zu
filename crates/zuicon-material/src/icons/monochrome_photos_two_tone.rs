// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MonochromePhotosTwoTone)]
pub fn monochrome_photos_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MonochromePhotosTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M12 18v-1.8c-1.8 0-3.2-1.4-3.2-3.2s1.4-3.2 3.2-3.2V8c-2.8 0-5 2.2-5 5s2.2 5 5 5zm5-5c0 2.8-2.2 5-5 5v1h8V7h-8v1c2.8 0 5 2.2 5 5z" opacity=".3"/><path d="M4 21h16c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2h-3.2L15 3H9L7.2 5H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2zm8-13V7h8v12h-8v-1c-2.8 0-5-2.2-5-5s2.2-5 5-5zm3.2 5c0 1.8-1.4 3.2-3.2 3.2V18c2.8 0 5-2.2 5-5s-2.2-5-5-5v1.8c1.8 0 3.2 1.4 3.2 3.2zm-6.4 0c0 1.8 1.4 3.2 3.2 3.2V9.8c-1.8 0-3.2 1.4-3.2 3.2z"/>
        </SvgIcon>
    }
}
