// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CleaningServicesSharp)]
pub fn cleaning_services_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CleaningServicesSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15,11V1H9v10H3v12h18V11H15z M19,21h-2v-4h-2v4h-2v-4h-2v4H9v-4H7v4H5v-8h14V21z"/>
        </SvgIcon>
    }
}
