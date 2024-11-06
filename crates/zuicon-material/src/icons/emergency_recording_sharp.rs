// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EmergencyRecordingSharp)]
pub fn emergency_recording_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EmergencyRecordingSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,10.48V4H2v16h16v-6.48l4,3.98v-11L18,10.48z M12,12l3,1.73l-1,1.73l-3-1.73V17H9v-3.27l-3,1.73l-1-1.73L8,12l-3-1.73 l1-1.73l3,1.73V7h2v3.27l3-1.73l1,1.73L12,12z"/>
        </SvgIcon>
    }
}
