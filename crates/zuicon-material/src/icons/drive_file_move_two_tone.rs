// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DriveFileMoveTwoTone)]
pub fn drive_file_move_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DriveFileMoveTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,6h-8l-2-2H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8C22,6.9,21.1,6,20,6z M20,18H4V6h5.17l1.41,1.41 L11.17,8H20V18z"/><path d="M10.59,7.41L9.17,6H4v12h16V8h-8.83L10.59,7.41z M12,9l4,4l-4,4v-3H8v-2h4V9z" opacity=".3"/>
        </SvgIcon>
    }
}
