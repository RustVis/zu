// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RollerSkatingSharp)]
pub fn roller_skating_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RollerSkatingSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,16l-0.01-6l-5.71-1.43C13.4,8.35,12.7,7.76,12.32,7H9V6h3.02L12,5H9V4h3V1H4v15H20z M5,23c-1.66,0-3-1.34-3-3 s1.34-3,3-3s3,1.34,3,3S6.66,23,5,23z M19,23c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3S20.66,23,19,23z M12,23 c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3S13.66,23,12,23z"/>
        </SvgIcon>
    }
}
