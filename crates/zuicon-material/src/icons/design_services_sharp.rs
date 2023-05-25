// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DesignServicesSharp)]
pub fn design_services_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DesignServicesSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.79,17.06l-5.55-5.55l1.57-1.57l-3.75-3.75l-1.57,1.57L6.94,2.21L2.21,6.94l5.55,5.55L3,17.25V21h3.75l4.76-4.76 l5.55,5.55l0,0v0L21.79,17.06z M9.18,11.07L5.04,6.94l1.9-1.9l1.27,1.27L7.02,7.5l1.41,1.41l1.19-1.19l1.45,1.45L9.18,11.07z M12.93,14.82l1.9-1.9l1.45,1.45l-1.19,1.19l1.41,1.41l1.19-1.19l1.27,1.27l-1.9,1.9L12.93,14.82z"/>
        </SvgIcon>
    }
}
