// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EmojiObjectsSharp)]
pub fn emoji_objects_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EmojiObjectsSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,3c-0.42,0-0.85,0.04-1.28,0.11c-2.81,0.5-5.08,2.75-5.6,5.55c-0.48,2.61,0.48,5.01,2.22,6.56 C7.77,15.6,8,16.13,8,16.69C8,18.21,8,21,8,21h2.28c0.35,0.6,0.98,1,1.72,1s1.38-0.4,1.72-1H16v-4.31c0-0.55,0.22-1.09,0.64-1.46 C18.09,13.95,19,12.08,19,10C19,6.13,15.87,3,12,3z M14,19h-4v-1h4V19z M14,17h-4v-1h4V17z M12.5,11.41V14h-1v-2.59L9.67,9.59 l0.71-0.71L12,10.5l1.62-1.62l0.71,0.71L12.5,11.41z"/>
        </SvgIcon>
    }
}
