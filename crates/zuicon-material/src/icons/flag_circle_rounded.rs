// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FlagCircleRounded)]
pub fn flag_circle_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FlagCircleRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M17,15h-3.38 c-0.38,0-0.73-0.21-0.89-0.55L12,13H9.5v4.25C9.5,17.66,9.16,18,8.75,18h0C8.34,18,8,17.66,8,17.25V8c0-0.55,0.45-1,1-1h4.38 c0.38,0,0.73,0.21,0.89,0.55L15,9h2c0.55,0,1,0.45,1,1v4C18,14.55,17.55,15,17,15z"/>
        </SvgIcon>
    }
}
