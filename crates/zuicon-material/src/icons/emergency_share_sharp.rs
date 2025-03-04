// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EmergencyShareSharp)]
pub fn emergency_share_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EmergencyShareSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,9c-3.15,0-6,2.41-6,6.15c0,2.49,2,5.44,6,8.85c4-3.41,6-6.36,6-8.85C18,11.41,15.15,9,12,9z M12,16.5 c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S12.83,16.5,12,16.5z M12,4c1.93,0,3.68,0.78,4.95,2.05l-1.41,1.41 C14.63,6.56,13.38,6,12,6S9.37,6.56,8.46,7.46L7.05,6.05C8.32,4.78,10.07,4,12,4z M19.78,3.23l-1.41,1.41 C16.74,3.01,14.49,2,12.01,2S7.27,3.01,5.64,4.63L4.22,3.22C6.22,1.23,8.97,0,12.01,0S17.79,1.23,19.78,3.23z"/>
        </SvgIcon>
    }
}
