// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhotoAlbumTwoTone)]
pub fn photo_album_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PhotoAlbumTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16,4v7l-2.5-1.5L11,11V4H6v16h12V4H16z M7,18l2.38-3.17L11,17l2.62-3.5L17,18H7z" opacity=".3"/><path d="M18,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C20,2.9,19.1,2,18,2z M18,20H6V4h5v7l2.5-1.5L16,11V4h2V20 z M13.62,13.5L17,18H7l2.38-3.17L11,17L13.62,13.5z"/>
        </SvgIcon>
    }
}
