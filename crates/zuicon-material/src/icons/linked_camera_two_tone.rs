// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LinkedCameraTwoTone)]
pub fn linked_camera_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LinkedCameraTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M20 20H4V8h4.05l1.83-2H15V4H9L7.17 6H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V9h-2v11zM16 2v1.33c2.58 0 4.66 2.09 4.67 4.66H22C21.99 4.68 19.31 2 16 2zm0 2.67V6c1.11 0 1.99.89 2 1.99h1.33c-.01-1.84-1.49-3.32-3.33-3.32z"/><path d="M14.98 10.01c-.13-.09-.26-.18-.39-.26.14.08.27.17.39.26zM17 9c0-.37-.11-.71-.28-1.01-.18-.3-.43-.55-.73-.72C15.7 7.1 15.36 7 15 7V6H9.88L8.05 8H4v12h16V9h-3zm-5 10c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5z" opacity=".3"/><path d="M12 9c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zm0 8c-1.65 0-3-1.35-3-3s1.35-3 3-3 3 1.35 3 3-1.35 3-3 3z"/>
        </SvgIcon>
    }
}
