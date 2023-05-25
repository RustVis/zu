// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AutoStoriesSharp)]
pub fn auto_stories_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AutoStoriesSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,4.6v2.12v7.24v2.66C19.86,16.21,18.69,16,17.5,16c-1.9,0-3.78,0.54-5.5,1.58v-3.62V7.79V5.48C10.38,4.55,8.51,4,6.5,4 S2.62,4.55,1,5.48V20c1.52-1.18,3.43-2,5.5-2s3.98,0.82,5.5,2c1.52-1.18,3.43-2,5.5-2s3.98,0.82,5.5,2V5.48 C22.37,5.12,21.7,4.84,21,4.6z"/>
        </SvgIcon>
    }
}
