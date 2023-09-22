// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AppRegistration)]
pub fn app_registration(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AppRegistration"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.88,11.29l-1.17-1.17c-0.16-0.16-0.42-0.16-0.58,0L18.25,11L20,12.75l0.88-0.88C21.04,11.71,21.04,11.45,20.88,11.29z"/>
        </SvgIcon>
    }
}
