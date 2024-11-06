// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WorkspacesOutlined)]
pub fn workspaces_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WorkspacesOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,15c1.1,0,2,0.9,2,2s-0.9,2-2,2s-2-0.9-2-2S4.9,15,6,15 M6,13c-2.2,0-4,1.8-4,4s1.8,4,4,4s4-1.8,4-4S8.2,13,6,13z M12,5 c1.1,0,2,0.9,2,2s-0.9,2-2,2s-2-0.9-2-2S10.9,5,12,5 M12,3C9.8,3,8,4.8,8,7s1.8,4,4,4s4-1.8,4-4S14.2,3,12,3z M18,15 c1.1,0,2,0.9,2,2s-0.9,2-2,2s-2-0.9-2-2S16.9,15,18,15 M18,13c-2.2,0-4,1.8-4,4s1.8,4,4,4s4-1.8,4-4S20.2,13,18,13z"/>
        </SvgIcon>
    }
}
