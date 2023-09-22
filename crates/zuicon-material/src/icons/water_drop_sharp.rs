// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WaterDropSharp)]
pub fn water_drop_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WaterDropSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2c-5.33,4.55-8,8.48-8,11.8c0,4.98,3.8,8.2,8,8.2s8-3.22,8-8.2C20,10.48,17.33,6.55,12,2z M13,18.91 C12.68,18.97,12.35,19,12,19c-2.69,0-4.88-1.94-5-5h1.5c0.08,2.07,1.5,3.5,3.5,3.5c0.35,0,0.69-0.04,1-0.13V18.91z"/>
        </SvgIcon>
    }
}
