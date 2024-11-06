// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ModeNightRounded)]
pub fn mode_night_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ModeNightRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M11.93,2.3C9.89,1.8,7.91,1.95,6.16,2.58C5.44,2.84,5.25,3.8,5.85,4.29C8.08,6.12,9.5,8.89,9.5,12 c0,3.11-1.42,5.88-3.65,7.71c-0.59,0.49-0.42,1.45,0.31,1.7C7.2,21.79,8.33,22,9.5,22c6.05,0,10.85-5.38,9.87-11.6 C18.76,6.48,15.78,3.24,11.93,2.3z"/>
        </SvgIcon>
    }
}
