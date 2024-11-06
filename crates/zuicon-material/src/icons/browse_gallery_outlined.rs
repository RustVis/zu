// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BrowseGalleryOutlined)]
pub fn browse_gallery_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BrowseGalleryOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9,3c-4.97,0-9,4.03-9,9s4.03,9,9,9s9-4.03,9-9S13.97,3,9,3z M9,19c-3.86,0-7-3.14-7-7s3.14-7,7-7s7,3.14,7,7 S12.86,19,9,19z"/><path d="M17.99,3.52v2.16C20.36,6.8,22,9.21,22,12c0,2.79-1.64,5.2-4.01,6.32v2.16C21.48,19.24,24,15.91,24,12 C24,8.09,21.48,4.76,17.99,3.52z"/>
        </SvgIcon>
    }
}
