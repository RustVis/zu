// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CarCrashSharp)]
pub fn car_crash_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CarCrashSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,1c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,1,18,1z M18.5,7h-1V3h1V7z M18.5,8v1h-1V8H18.5z M17.91,13 c0.06,0.16,0.09,0.33,0.09,0.5c0,0.83-0.67,1.5-1.5,1.5S15,14.33,15,13.5c0-0.39,0.15-0.74,0.39-1.01 c-1.63-0.66-2.96-1.91-3.71-3.49H5.81l1.04-3H11c0-0.69,0.1-1.37,0.29-2H5.41L3,11v9h3v-2h12v2h3v-7.68 C19.95,12.83,18.84,13.01,17.91,13z M7.5,15C6.67,15,6,14.33,6,13.5S6.67,12,7.5,12S9,12.67,9,13.5S8.33,15,7.5,15z"/>
        </SvgIcon>
    }
}
