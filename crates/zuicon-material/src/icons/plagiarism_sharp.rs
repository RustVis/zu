// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PlagiarismSharp)]
pub fn plagiarism_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PlagiarismSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,2H4v20h16V8L14,2z M15.04,19.45l-1.88-1.88c-1.33,0.71-3.01,0.53-4.13-0.59c-1.37-1.37-1.37-3.58,0-4.95 c1.37-1.37,3.58-1.37,4.95,0c1.12,1.12,1.31,2.8,0.59,4.13l1.88,1.88L15.04,19.45z M13,9V3.5L18.5,9H13z"/>
        </SvgIcon>
    }
}
