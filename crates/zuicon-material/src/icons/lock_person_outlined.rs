// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LockPersonOutlined)]
pub fn lock_person_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LockPersonOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,20V10h12v1c0.7,0,1.37,0.1,2,0.29V10c0-1.1-0.9-2-2-2h-1V6c0-2.76-2.24-5-5-5S7,3.24,7,6v2H6c-1.1,0-2,0.9-2,2v10 c0,1.1,0.9,2,2,2h6.26c-0.42-0.6-0.75-1.28-0.97-2H6z M9,6c0-1.66,1.34-3,3-3s3,1.34,3,3v2H9V6z"/><path d="M18,13c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,13,18,13z M18,15c0.83,0,1.5,0.67,1.5,1.5S18.83,18,18,18 s-1.5-0.67-1.5-1.5S17.17,15,18,15z M18,21c-1.03,0-1.94-0.52-2.48-1.32C16.25,19.26,17.09,19,18,19s1.75,0.26,2.48,0.68 C19.94,20.48,19.03,21,18,21z"/>
        </SvgIcon>
    }
}
