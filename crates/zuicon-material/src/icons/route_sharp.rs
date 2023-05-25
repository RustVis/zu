// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RouteSharp)]
pub fn route_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RouteSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,15.18V3h-8v16H7V8.82C8.16,8.4,9,7.3,9,6c0-1.66-1.34-3-3-3S3,4.34,3,6c0,1.3,0.84,2.4,2,2.82V21h8V5h4v10.18 c-1.16,0.41-2,1.51-2,2.82c0,1.66,1.34,3,3,3s3-1.34,3-3C21,16.7,20.16,15.6,19,15.18z"/>
        </SvgIcon>
    }
}
