// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PlaylistAddCircle)]
pub fn playlist_add_circle(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PlaylistAddCircle"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M7,7h7v2H7V7z M10,15H7v-2h3V15z M7,12v-2h7v2H7z M19,15h-2v2h-2v-2h-2v-2h2v-2h2v2h2V15z"/>
        </SvgIcon>
    }
}
