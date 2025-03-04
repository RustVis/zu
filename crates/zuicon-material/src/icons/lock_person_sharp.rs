// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LockPersonSharp)]
pub fn lock_person_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LockPersonSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16.43,11.18c1.26-0.29,2.47-0.21,3.57,0.12V8h-3V6.22c0-2.61-1.91-4.94-4.51-5.19C9.51,0.74,7,3.08,7,6v2H4v14h8.26 c-1.01-1.45-1.5-3.3-1.15-5.27C11.6,14,13.74,11.79,16.43,11.18z M8.9,6c0-1.71,1.39-3.1,3.1-3.1s3.1,1.39,3.1,3.1v2H8.9V6z"/><path d="M18,13c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,13,18,13z M18,15c0.83,0,1.5,0.67,1.5,1.5S18.83,18,18,18 s-1.5-0.67-1.5-1.5S17.17,15,18,15z M18,21c-1.03,0-1.94-0.52-2.48-1.32C16.25,19.26,17.09,19,18,19s1.75,0.26,2.48,0.68 C19.94,20.48,19.03,21,18,21z"/>
        </SvgIcon>
    }
}
