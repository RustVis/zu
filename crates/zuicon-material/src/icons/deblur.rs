// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Deblur)]
pub fn deblur(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Deblur"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,3v18c4.97,0,9-4.03,9-9C21,7.03,16.97,3,12,3z"/>
        </SvgIcon>
    }
}
