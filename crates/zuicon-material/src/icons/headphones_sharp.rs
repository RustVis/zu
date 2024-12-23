// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HeadphonesSharp)]
pub fn headphones_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HeadphonesSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,3c-4.97,0-9,4.03-9,9v9h6v-8H5v-1c0-3.87,3.13-7,7-7s7,3.13,7,7v1h-4v8h6v-9C21,7.03,16.97,3,12,3z"/>
        </SvgIcon>
    }
}
