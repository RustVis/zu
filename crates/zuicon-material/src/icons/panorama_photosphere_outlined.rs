// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PanoramaPhotosphereOutlined)]
pub fn panorama_photosphere_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PanoramaPhotosphereOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.95,8.15c-0.29-0.16-0.61-0.31-0.93-0.46C19.4,4.33,15.98,2,12,2C8.02,2,4.6,4.33,2.99,7.68 c-0.33,0.15-0.64,0.3-0.93,0.46C1.41,8.5,1,9.17,1,9.91v4.18c0,0.74,0.41,1.41,1.05,1.77c0.29,0.16,0.61,0.31,0.93,0.46 C4.6,19.67,8.02,22,12,22c3.98,0,7.4-2.33,9.01-5.68c0.33-0.15,0.64-0.3,0.93-0.46C22.59,15.5,23,14.83,23,14.09V9.91 C23,9.17,22.59,8.5,21.95,8.15z M21,9.91C21,9.91,21,9.91,21,9.91l0,4.19C18.81,15.31,15.53,16,12,16c-3.53,0-6.81-0.7-9-1.91 c0,0,0,0,0,0l0-4.18C5.2,8.69,8.47,8,12,8C15.53,8,18.81,8.7,21,9.91z M12,4c2.37,0,4.49,1.04,5.95,2.68C16.17,6.25,14.15,6,12,6 C9.85,6,7.83,6.25,6.05,6.68C7.51,5.04,9.63,4,12,4z M12,20c-2.37,0-4.49-1.04-5.95-2.68C7.83,17.75,9.85,18,12,18 s4.17-0.25,5.95-0.68C16.49,18.96,14.37,20,12,20z"/>
        </SvgIcon>
    }
}
