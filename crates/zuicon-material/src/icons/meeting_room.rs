// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MeetingRoom)]
pub fn meeting_room(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MeetingRoom"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14 6v15H3v-2h2V3h9v1h5v15h2v2h-4V6h-3zm-4 5v2h2v-2h-2z"/>
        </SvgIcon>
    }
}
