// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhotoAlbumSharp)]
pub fn photo_album_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PhotoAlbumSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,2H4v20h16V2z M11,4h5v7l-2.5-1.5L11,11V4z M7,18l2.38-3.17L11,17l2.62-3.5L17,18H7z"/>
        </SvgIcon>
    }
}
