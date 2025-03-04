// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EnergySavingsLeafRounded)]
pub fn energy_savings_leaf_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EnergySavingsLeafRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,3c-4.8,0-9,3.86-9,9c0,2.12,0.74,4.07,1.97,5.61l-1.68,1.68c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.41,0 l1.68-1.68C7.93,20.26,9.88,21,12,21c2.3,0,4.61-0.88,6.36-2.64C20.12,16.61,21,14.3,21,12V5c0-1.1-0.9-2-2-2H12z M15.83,12.26 l-5.16,4.63c-0.16,0.15-0.41,0.14-0.56-0.01c-0.14-0.14-0.16-0.36-0.04-0.52l2.44-3.33l-4.05-0.4c-0.44-0.04-0.63-0.59-0.3-0.89 l5.16-4.63c0.16-0.15,0.41-0.14,0.56,0.01c0.14,0.14,0.16,0.36,0.04,0.52l-2.44,3.33l4.05,0.4 C15.98,11.41,16.16,11.96,15.83,12.26z"/>
        </SvgIcon>
    }
}
