// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DonutSmallSharp)]
pub fn donut_small_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DonutSmallSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M13 9.18c.85.3 1.51.97 1.82 1.82h7.13c-.47-4.72-4.23-8.48-8.95-8.95v7.13zm-2 5.64C9.84 14.4 9 13.3 9 12c0-1.3.84-2.4 2-2.82V2.05c-5.05.5-9 4.76-9 9.95 0 5.19 3.95 9.45 9 9.95v-7.13zM14.82 13c-.3.85-.97 1.51-1.82 1.82v7.13c4.72-.47 8.48-4.23 8.95-8.95h-7.13z"/>
        </SvgIcon>
    }
}
