// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PlaylistAddCheckRounded)]
pub fn playlist_add_check_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PlaylistAddCheckRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13 10H3c-.55 0-1 .45-1 1s.45 1 1 1h10c.55 0 1-.45 1-1s-.45-1-1-1zm0-4H3c-.55 0-1 .45-1 1s.45 1 1 1h10c.55 0 1-.45 1-1s-.45-1-1-1zM3 16h6c.55 0 1-.45 1-1s-.45-1-1-1H3c-.55 0-1 .45-1 1s.45 1 1 1zm19.21-3.79l.09.09c.39.39.39 1.02 0 1.41l-5.58 5.59c-.39.39-1.02.39-1.41 0l-3.09-3.09c-.39-.39-.39-1.02 0-1.41l.09-.09c.39-.39 1.02-.39 1.41 0l2.3 2.3 4.78-4.79c.38-.4 1.02-.4 1.41-.01z"/>
        </SvgIcon>
    }
}
