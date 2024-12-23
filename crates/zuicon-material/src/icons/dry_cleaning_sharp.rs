// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DryCleaningSharp)]
pub fn dry_cleaning_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DryCleaningSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,12l-8-3.56V6h-1c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1h2c0-1.84-1.66-3.3-3.56-2.95 C10.26,2.27,9.29,3.22,9.06,4.4C8.76,5.96,9.66,7.34,11,7.82v0.63l-8,3.56L3,16h4v6h10v-6h4V12z M19,14h-2v-1H7v1H5v-0.7l7-3.11 l7,3.11V14z"/>
        </SvgIcon>
    }
}
