// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AddLocationOutlined)]
pub fn add_location_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AddLocationOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13,6v3h3v2h-3v3h-2v-3H8V9h3V6H13z M18,10.2C18,6.57,15.35,4,12,4s-6,2.57-6,6.2 c0,2.34,1.95,5.44,6,9.14C16.05,15.64,18,12.54,18,10.2z M12,2c4.2,0,8,3.22,8,8.2c0,3.32-2.67,7.25-8,11.8 c-5.33-4.55-8-8.48-8-11.8C4,5.22,7.8,2,12,2z"/>
        </SvgIcon>
    }
}
