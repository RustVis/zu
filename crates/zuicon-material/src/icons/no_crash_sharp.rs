// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoCrashSharp)]
pub fn no_crash_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NoCrashSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18.57,8H5.43L3,15v9h3v-2h12v2h3v-9L18.57,8z M6.85,10h10.29l1.04,3H5.81L6.85,10z M6,17.5C6,16.67,6.67,16,7.5,16 S9,16.67,9,17.5S8.33,19,7.5,19S6,18.33,6,17.5z M15,17.5c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S17.33,19,16.5,19 S15,18.33,15,17.5z M12,6.36L9.17,3.54l1.41-1.41L12,3.54L15.54,0l1.41,1.41L12,6.36z"/>
        </SvgIcon>
    }
}
