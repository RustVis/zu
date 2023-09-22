// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SosOutlined)]
pub fn sos_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SosOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.5,7h-3c-1.1,0-2,0.9-2,2v6c0,1.1,0.9,2,2,2h3c1.1,0,2-0.9,2-2V9C15.5,7.9,14.6,7,13.5,7z M13.5,15h-3V9h3V15z M1,15h4 v-2H3c-1.1,0-2-0.9-2-2V9c0-1.1,0.9-2,2-2h4v2H3v2h2c1.1,0,2,0.9,2,2v2c0,1.1-0.9,2-2,2H1V15z M17,15h4v-2h-2c-1.1,0-2-0.9-2-2V9 c0-1.1,0.9-2,2-2h4v2h-4v2h2c1.1,0,2,0.9,2,2v2c0,1.1-0.9,2-2,2h-4V15z"/>
        </SvgIcon>
    }
}
