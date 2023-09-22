// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OutlinedFlagRounded)]
pub fn outlined_flag_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OutlinedFlagRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M14 6l-.72-1.45c-.17-.34-.52-.55-.9-.55H6c-.55 0-1 .45-1 1v15c0 .55.45 1 1 1s1-.45 1-1v-6h5l.72 1.45c.17.34.52.55.89.55H19c.55 0 1-.45 1-1V7c0-.55-.45-1-1-1h-5zm4 8h-4l-1-2H7V6h5l1 2h5v6z"/>
        </SvgIcon>
    }
}
