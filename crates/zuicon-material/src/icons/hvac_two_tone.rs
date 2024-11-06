// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HvacTwoTone)]
pub fn hvac_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HvacTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,19h14V5H5V19z M12,6c3.31,0,6,2.69,6,6s-2.69,6-6,6s-6-2.69-6-6S8.69,6,12,6z" opacity=".3"/><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,19H5V5h14V19z"/><path d="M12,18c3.31,0,6-2.69,6-6s-2.69-6-6-6s-6,2.69-6,6S8.69,18,12,18z M12,16c-1.01,0-1.91-0.39-2.62-1h5.24 C13.91,15.61,13.01,16,12,16z M12,8c1.01,0,1.91,0.39,2.62,1H9.38C10.09,8.39,10.99,8,12,8z M8.56,10h6.89 c0.26,0.45,0.44,0.96,0.51,1.5h-7.9C8.12,10.96,8.29,10.45,8.56,10z M15.95,12.5c-0.07,0.54-0.24,1.05-0.51,1.5H8.56 c-0.26-0.45-0.44-0.96-0.51-1.5H15.95z"/>
        </SvgIcon>
    }
}
