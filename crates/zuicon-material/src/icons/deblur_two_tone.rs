// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DeblurTwoTone)]
pub fn deblur_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DeblurTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,3v2c3.86,0,7,3.14,7,7s-3.14,7-7,7v2c4.96,0,9-4.04,9-9S16.96,3,12,3z"/><path d="M12,5v14c3.86,0,7-3.14,7-7S15.86,5,12,5z" opacity=".3"/>
        </SvgIcon>
    }
}
