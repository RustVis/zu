// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TimesOneMobiledataRounded)]
pub fn times_one_mobiledata_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TimesOneMobiledataRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M5,7h2c0.55,0,1,0.45,1,1v8c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V9H5C4.45,9,4,8.55,4,8v0C4,7.45,4.45,7,5,7z M15.83,11.72l1.92-3.21C18.15,7.85,17.67,7,16.9,7h0c-0.35,0-0.68,0.18-0.86,0.49l-1.37,2.28l-1.38-2.29 C13.11,7.18,12.79,7,12.44,7h0c-0.78,0-1.26,0.85-0.86,1.51l1.92,3.21l-2.26,3.77c-0.4,0.67,0.08,1.51,0.86,1.51h0 c0.35,0,0.68-0.18,0.86-0.49l1.71-2.85l1.71,2.85c0.18,0.3,0.51,0.49,0.86,0.49h0.01c0.78,0,1.26-0.85,0.86-1.51L15.83,11.72z"/>
        </SvgIcon>
    }
}
