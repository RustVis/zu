// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WashSharp)]
pub fn wash_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WashSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9.12,5L1,12.68V23h18v-2.5h-7v-1h9V17h-9v-1h10v-2.5H12v-1h8V10H8.86l1.88-3.3L9.12,5L9.12,5z M13.5,9 C14.33,9,15,8.33,15,7.5C15,6.66,13.5,5,13.5,5S12,6.66,12,7.5C12,8.33,12.67,9,13.5,9z M18.5,1c0,0-2.5,2.83-2.5,4.5 C16,6.88,17.12,8,18.5,8S21,6.88,21,5.5C21,3.83,18.5,1,18.5,1z"/>
        </SvgIcon>
    }
}
