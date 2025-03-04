// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AppsOutageRounded)]
pub fn apps_outage_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AppsOutageRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,8c1.1,0,2-0.9,2-2S7.1,4,6,4S4,4.9,4,6S4.9,8,6,8z M12,20c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S10.9,20,12,20z M6,20 c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S4.9,20,6,20z M6,14c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S4.9,14,6,14z M12,14 c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S10.9,14,12,14z M12.07,4C12.05,4,12.02,4,12,4c-1.1,0-2,0.9-2,2s0.9,2,2,2 c0.22,0,0.43-0.04,0.63-0.1C12.22,7.01,12,6.03,12,5C12,4.66,12.02,4.33,12.07,4z M19,12c-1.03,0-2.01-0.22-2.9-0.63 C16.04,11.57,16,11.78,16,12c0,1.1,0.9,2,2,2s2-0.9,2-2c0-0.02,0-0.05,0-0.07C19.67,11.98,19.34,12,19,12z M18,20c1.1,0,2-0.9,2-2 s-0.9-2-2-2s-2,0.9-2,2S16.9,20,18,20z M19,0c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S21.76,0,19,0z M19.5,7.5 C19.5,7.78,19.28,8,19,8c-0.27,0-0.5-0.22-0.5-0.5S18.72,7,19,7S19.5,7.22,19.5,7.5z M19,6c-0.28,0-0.5-0.22-0.5-0.5v-3 C18.5,2.22,18.72,2,19,2s0.5,0.22,0.5,0.5v3C19.5,5.78,19.28,6,19,6z"/>
        </SvgIcon>
    }
}
