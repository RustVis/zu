// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MonochromePhotosRounded)]
pub fn monochrome_photos_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MonochromePhotosRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M20 5h-3.2l-1.2-1.34c-.38-.42-.92-.66-1.49-.66H9.89c-.57 0-1.11.24-1.49.66L7.2 5H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 13c0 .55-.45 1-1 1h-7v-1c-2.8 0-5-2.2-5-5s2.2-5 5-5V7h7c.55 0 1 .45 1 1v10zm-3-5c0-2.8-2.2-5-5-5v1.8c1.8 0 3.2 1.4 3.2 3.2s-1.4 3.2-3.2 3.2V18c2.8 0 5-2.2 5-5zm-8.2 0c0 1.8 1.4 3.2 3.2 3.2V9.8c-1.8 0-3.2 1.4-3.2 3.2z"/>
        </SvgIcon>
    }
}
