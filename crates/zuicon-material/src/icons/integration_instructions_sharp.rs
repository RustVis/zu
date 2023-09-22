// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(IntegrationInstructionsSharp)]
pub fn integration_instructions_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("IntegrationInstructionsSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,3h-6.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H3v18h18V3z M11,14.17l-1.41,1.42L6,12l3.59-3.59L11,9.83L8.83,12 L11,14.17z M12,4.25c-0.41,0-0.75-0.34-0.75-0.75S11.59,2.75,12,2.75s0.75,0.34,0.75,0.75S12.41,4.25,12,4.25z M14.41,15.59 L13,14.17L15.17,12L13,9.83l1.41-1.42L18,12L14.41,15.59z"/>
        </SvgIcon>
    }
}
