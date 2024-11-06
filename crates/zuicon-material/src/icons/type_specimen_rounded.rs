// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TypeSpecimenRounded)]
pub fn type_specimen_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TypeSpecimenRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,6L3,6C2.45,6,2,6.45,2,7v13c0,1.1,0.9,2,2,2h13c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H4V7C4,6.45,3.55,6,3,6z"/><path d="M20,2H8C6.9,2,6,2.9,6,4v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M16.46,14.01l-0.63-1.82h-3.63 l-0.65,1.82c-0.1,0.29-0.38,0.48-0.68,0.48h0c-0.51,0-0.86-0.51-0.68-0.98l2.73-7.27C13.08,5.8,13.52,5.5,14,5.5h0 c0.48,0,0.92,0.3,1.09,0.75l2.73,7.27c0.18,0.47-0.17,0.98-0.68,0.98h0C16.83,14.5,16.56,14.31,16.46,14.01z"/>
        </SvgIcon>
    }
}
