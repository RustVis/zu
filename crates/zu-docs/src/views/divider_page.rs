// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::divider::Divider;

#[function_component(DividerPage)]
pub fn divider_page() -> Html {
    // TODO(Shaohua): Add List, ListItem and ListItemText
    html! {
        <div class="container">
        <h1>{"Divider"}</h1>
        <p>{"Dividers separate content into clear groups."}</p>

        <h2>{"List dividers"}</h2>
        <p>{"The divider renders as an <hr> by default.
         You can save rendering this DOM element by using the divider prop on the ListItem component."}</p>
        <div class="demo-box">
            <nav>
                <div><span>{"Inbox"}</span></div>
                <Divider></Divider>

                <div><span>{"Drafts"}</span></div>
                <Divider></Divider>

                <div><span>{"Trash"}</span></div>
                <Divider></Divider>

                <div><span>{"Spam"}</span></div>
                <Divider></Divider>
            </nav>
        </div>
        </div>
    }
}
