// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ContentPasteGoTwoTone)]
pub fn content_paste_go_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ContentPasteGoTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10,17c0-3.31,2.69-6,6-6h3V5h-2v3H7V5H5v14h5V17z" opacity=".3"/><path d="M10,19H5V5h2v3h10V5h2v6h2V5c0-1.1-0.9-2-2-2h-4.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H5C3.9,3,3,3.9,3,5v14 c0,1.1,0.9,2,2,2h5V19z M12,3c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S11.45,3,12,3z"/>
        </SvgIcon>
    }
}
