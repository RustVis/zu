// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::divider::{Divider, TextAlign, Variant};

use crate::components::demo_box::DemoBox;

#[function_component(DividerPage)]
pub fn divider_page() -> Html {
    // TODO(Shaohua): Add List, ListItem and ListItemText
    // TODO(Shaohua): Add vertical divider.

    html! {
        <div class="container">
        <h1>{"Divider"}</h1>
        <p>{"Dividers separate content into clear groups."}</p>

        <h2>{"List dividers"}</h2>
        <p>{"The divider renders as an <hr> by default.
         You can save rendering this DOM element by using the divider prop on the ListItem component."}</p>
        <DemoBox>
            <nav>
                <div><span>{"Inbox"}</span></div>
                <Divider />

                <div><span>{"Drafts"}</span></div>
                <Divider />

                <div><span>{"Trash"}</span></div>
                <Divider />

                <div><span>{"Spam"}</span></div>
                <Divider />
            </nav>
        </DemoBox>

        <h2>{"HTML5 specification"}</h2>
        <p>{"In a list, you should ensure the Divider is rendered as an <li> \
        to match the HTML5 specification. The examples below show two ways of achieving this."}</p>
        <h3>{"Inset dividers"}</h3>
        <DemoBox>
            <ul style="list-style-type: none">
                <li>{"Photos"}</li>
                <Divider variant={Variant::Inset} component="li" />
                <li>{"Work"}</li>
                <Divider variant={Variant::Inset} component="li" />
                <li>{"Vacation"}</li>
            </ul>
        </DemoBox>

        <h3>{"Dividers with text"}</h3>
        <p>{"You can also render a divider with content."}</p>
        <DemoBox>
            <div>
                <div>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus id dignissim justo.
            Nulla ut facilisis ligula. Interdum et malesuada fames ac ante ipsum primis in faucibus.
            Sed malesuada lobortis pretium."}</div>
                <Divider>{"CENTER"}</Divider>
                <div>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus id dignissim justo.
            Nulla ut facilisis ligula. Interdum et malesuada fames ac ante ipsum primis in faucibus.
            Sed malesuada lobortis pretium."}</div>
                <Divider text_align={TextAlign::Left}>{"Left"}</Divider>
                <div>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus id dignissim justo.
            Nulla ut facilisis ligula. Interdum et malesuada fames ac ante ipsum primis in faucibus.
            Sed malesuada lobortis pretium."}</div>
                <Divider text_align={TextAlign::Right}>{"Right"}</Divider>
                <div>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus id dignissim justo.
            Nulla ut facilisis ligula. Interdum et malesuada fames ac ante ipsum primis in faucibus.
            Sed malesuada lobortis pretium."}</div>
            </div>
        </DemoBox>

        </div>
    }
}
