// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EggSharp)]
pub fn egg_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EggSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,3C8.5,3,5,9.33,5,14c0,3.87,3.13,7,7,7s7-3.13,7-7C19,9.33,15.5,3,12,3z M13,18c-3,0-5-1.99-5-5c0-0.55,0-1,0-1h2 c0,0,0,1,0,1c0,2.92,2.42,3,3,3c0.55,0,1,0,1,0l0,2C14,18,13.55,18,13,18z"/>
        </SvgIcon>
    }
}
