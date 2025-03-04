// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhpRounded)]
pub fn php_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PhpRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6.5,10.5h-2v1h2V10.5z M20,10.5h-2v1h2V10.5z M13,12.5h-2v1.75c0,0.41-0.34,0.75-0.75,0.75S9.5,14.66,9.5,14.25v-4.5 C9.5,9.34,9.84,9,10.25,9S11,9.34,11,9.75V11h2V9.75C13,9.34,13.34,9,13.75,9s0.75,0.34,0.75,0.75v4.5c0,0.41-0.34,0.75-0.75,0.75 S13,14.66,13,14.25V12.5z M18,14.25c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75V10c0-0.55,0.45-1,1-1H20 c0.83,0,1.5,0.68,1.5,1.5v1c0,0.82-0.67,1.5-1.5,1.5h-2V14.25z M3,10c0-0.55,0.45-1,1-1h2.5C7.33,9,8,9.68,8,10.5v1 C8,12.32,7.33,13,6.5,13h-2v1.25C4.5,14.66,4.16,15,3.75,15S3,14.66,3,14.25V10z"/>
        </SvgIcon>
    }
}
