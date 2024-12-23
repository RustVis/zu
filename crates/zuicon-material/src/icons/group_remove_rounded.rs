// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(GroupRemoveRounded)]
pub fn group_remove_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("GroupRemoveRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,10c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1s-0.45,1-1,1h-4C18.45,11,18,10.55,18,10z M8,4C5.79,4,4,5.79,4,8s1.79,4,4,4 s4-1.79,4-4S10.21,4,8,4z M8,13c-2.67,0-8,1.34-8,4v3h16v-3C16,14.34,10.67,13,8,13z M12.51,4.05C13.43,5.11,14,6.49,14,8 s-0.57,2.89-1.49,3.95C14.47,11.7,16,10.04,16,8S14.47,4.3,12.51,4.05z M16.53,13.83C17.42,14.66,18,15.7,18,17v3h2v-3 C20,15.55,18.41,14.49,16.53,13.83z"/>
        </SvgIcon>
    }
}
