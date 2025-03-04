// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RollerSkating)]
pub fn roller_skating(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RollerSkating"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9,6.5C9,6.22,9.22,6,9.5,6h2.52L12,5H9.5C9.22,5,9,4.78,9,4.5C9,4.22,9.22,4,9.5,4H12V1H4v15h3h13l0-2.88 c0-2.1-1.55-3.53-3.03-3.88l-2.7-0.67C13.4,8.35,12.7,7.76,12.32,7H9.5C9.22,7,9,6.78,9,6.5z M5,23c-1.66,0-3-1.34-3-3s1.34-3,3-3 s3,1.34,3,3S6.66,23,5,23z M19,23c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3S20.66,23,19,23z M12,23c-1.66,0-3-1.34-3-3 s1.34-3,3-3s3,1.34,3,3S13.66,23,12,23z"/>
        </SvgIcon>
    }
}
