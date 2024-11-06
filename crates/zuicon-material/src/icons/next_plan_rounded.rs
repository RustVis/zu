// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NextPlanRounded)]
pub fn next_plan_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NextPlanRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M18,13.97h-5l2.26-2.26 c-0.91-1.06-2.25-1.74-3.76-1.74c-2.37,0-4.35,1.66-4.86,3.88l-0.96-0.32c0.64-2.62,3-4.56,5.82-4.56c1.78,0,3.37,0.79,4.47,2.03 L18,8.97V13.97z"/>
        </SvgIcon>
    }
}
