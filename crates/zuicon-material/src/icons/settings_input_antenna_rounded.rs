// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SettingsInputAntennaRounded)]
pub fn settings_input_antenna_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SettingsInputAntennaRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,5c-3.48,0-6.37,2.54-6.91,5.87c-0.1,0.59,0.39,1.13,1,1.13c0.49,0,0.9-0.36,0.98-0.85C7.48,8.79,9.53,7,12,7 s4.52,1.79,4.93,4.15c0.08,0.49,0.49,0.85,0.98,0.85c0.61,0,1.09-0.54,0.99-1.13C18.37,7.54,15.48,5,12,5z M13,14.29 c1.07-0.48,1.76-1.66,1.41-2.99c-0.22-0.81-0.87-1.47-1.68-1.7C11.04,9.12,9.5,10.38,9.5,12c0,1.02,0.62,1.9,1.5,2.29v3.3 l-2.71,2.7c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.41,0l2.3-2.3l2.3,2.3c0.39,0.39,1.02,0.39,1.41,0s0.39-1.02,0-1.41 L13,17.59V14.29z M12,1C6.3,1,1.61,5.34,1.05,10.9C1,11.49,1.46,12,2.05,12c0.51,0,0.94-0.38,0.99-0.88C3.48,6.56,7.33,3,12,3 s8.52,3.56,8.96,8.12c0.05,0.5,0.48,0.88,0.99,0.88c0.59,0,1.06-0.51,1-1.1C22.39,5.34,17.7,1,12,1z"/>
        </SvgIcon>
    }
}
