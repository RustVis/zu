// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SpatialAudioOffRounded)]
pub fn spatial_audio_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SpatialAudioOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16.39,15.56C14.71,14.7,12.53,14,10,14c-2.53,0-4.71,0.7-6.39,1.56C2.61,16.07,2,17.1,2,18.22L2,19c0,1.1,0.9,2,2,2h12 c1.1,0,2-0.9,2-2l0-0.78C18,17.1,17.39,16.07,16.39,15.56z"/><path d="M21.11,1.85c-0.37-0.48-1.08-0.52-1.5-0.09l0,0c-0.35,0.35-0.39,0.91-0.09,1.3c1.17,1.5,2.64,5.23,0,8.61 c-0.3,0.39-0.26,0.95,0.09,1.3l0,0c0.43,0.43,1.13,0.38,1.5-0.09C22.61,10.95,24.46,6.16,21.11,1.85z"/><path d="M18.31,4.84c-0.33-0.57-1.11-0.67-1.58-0.21c-0.33,0.33-0.36,0.84-0.13,1.25c0.25,0.44,0.74,1.69-0.01,2.99 c-0.23,0.4-0.19,0.9,0.14,1.22l0,0c0.47,0.47,1.25,0.35,1.58-0.22C19.47,7.88,18.89,5.85,18.31,4.84z"/>
        </SvgIcon>
    }
}
