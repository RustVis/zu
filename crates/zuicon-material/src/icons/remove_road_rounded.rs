// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RemoveRoadRounded)]
pub fn remove_road_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RemoveRoadRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,4L19,4c-0.55,0-1,0.45-1,1v7c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V5C20,4.45,19.55,4,19,4z"/><path d="M5,20L5,20c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1h0C4.45,4,4,4.45,4,5v14C4,19.55,4.45,20,5,20z"/><path d="M12,8L12,8c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2C11,7.55,11.45,8,12,8z"/><path d="M12,14L12,14c0.55,0,1-0.45,1-1v-2c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2C11,13.55,11.45,14,12,14z"/><path d="M12,20L12,20c0.55,0,1-0.45,1-1v-2c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2C11,19.55,11.45,20,12,20z"/><path d="M21.79,15.71L21.79,15.71c-0.39-0.39-1.02-0.39-1.41,0L19,17.09l-1.38-1.38c-0.39-0.39-1.02-0.39-1.41,0v0 c-0.39,0.39-0.39,1.02,0,1.41l1.38,1.38l-1.38,1.38c-0.39,0.39-0.39,1.02,0,1.41l0,0c0.39,0.39,1.02,0.39,1.41,0L19,19.91 l1.38,1.38c0.39,0.39,1.02,0.39,1.41,0l0,0c0.39-0.39,0.39-1.02,0-1.41l-1.38-1.38l1.38-1.38C22.18,16.73,22.18,16.1,21.79,15.71z"/>
        </SvgIcon>
    }
}
