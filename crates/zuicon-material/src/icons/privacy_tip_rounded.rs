// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PrivacyTipRounded)]
pub fn privacy_tip_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PrivacyTipRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4.19,4.47C3.47,4.79,3,5.51,3,6.3V11c0,5.55,3.84,10.74,9,12c5.16-1.26,9-6.45,9-12V6.3c0-0.79-0.47-1.51-1.19-1.83 l-7-3.11c-0.52-0.23-1.11-0.23-1.62,0L4.19,4.47z M12,7L12,7c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v0 C11,7.45,11.45,7,12,7z M12,11L12,11c0.55,0,1,0.45,1,1v4c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v-4C11,11.45,11.45,11,12,11z"/>
        </SvgIcon>
    }
}
