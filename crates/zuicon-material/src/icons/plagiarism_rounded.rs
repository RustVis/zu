// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PlagiarismRounded)]
pub fn plagiarism_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PlagiarismRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19.41,7.41l-4.83-4.83C14.21,2.21,13.7,2,13.17,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V8.83 C20,8.3,19.79,7.79,19.41,7.41z M15.74,18.74L15.74,18.74c-0.39,0.39-1.02,0.39-1.41,0l-1.18-1.18c-1.33,0.71-3.01,0.53-4.13-0.59 c-1.52-1.52-1.35-4.08,0.5-5.37c1.16-0.81,2.78-0.81,3.95,0c1.55,1.08,1.9,3.04,1.09,4.55l1.18,1.18 C16.13,17.72,16.13,18.35,15.74,18.74z M14,9c-0.55,0-1-0.45-1-1V3.5L18.5,9H14z"/>
        </SvgIcon>
    }
}
