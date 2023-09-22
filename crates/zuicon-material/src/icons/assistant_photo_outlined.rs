// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AssistantPhotoOutlined)]
pub fn assistant_photo_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AssistantPhotoOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M12.36 6l.08.39.32 1.61H18v6h-3.36l-.08-.39-.32-1.61H7V6h5.36M14 4H5v17h2v-7h5.6l.4 2h7V6h-5.6L14 4z"/>
        </SvgIcon>
    }
}
