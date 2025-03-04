// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RunCircleRounded)]
pub fn run_circle_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RunCircleRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M13.5,6c0.55,0,1,0.45,1,1 c0,0.55-0.45,1-1,1s-1-0.45-1-1C12.5,6.45,12.95,6,13.5,6z M15.41,11.91c-0.71-0.2-1.63-0.74-2.32-1.66l-0.41,2.35l1.19,1.3 C13.95,13.98,14,14.1,14,14.22v3.28c0,0.28-0.22,0.5-0.5,0.5h0c-0.28,0-0.5-0.22-0.5-0.5v-3.08l-1.11-1.21l-0.43,2.15 c-0.05,0.27-0.32,0.45-0.59,0.39l-2.78-0.57c-0.27-0.06-0.45-0.32-0.39-0.59v0c0.06-0.27,0.32-0.44,0.59-0.39l2.29,0.47l0.96-4.89 L10,10.35v1.15c0,0.28-0.22,0.5-0.5,0.5h0C9.22,12,9,11.78,9,11.5V10c0-0.21,0.13-0.4,0.33-0.47l2.95-1.09 c0.49-0.18,1.02,0.04,1.25,0.51c0.65,1.35,1.55,1.85,2.1,2C15.85,11,16,11.18,16,11.4v0.04C16,11.75,15.71,11.99,15.41,11.91z"/>
        </SvgIcon>
    }
}
