// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BallotSharp)]
pub fn ballot_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BallotSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M13 9.5h5v-2h-5v2zm0 7h5v-2h-5v2zm8 4.5H3V3h18v18zM6 11h5V6H6v5zm1-4h3v3H7V7zM6 18h5v-5H6v5zm1-4h3v3H7v-3z"/>
        </SvgIcon>
    }
}
