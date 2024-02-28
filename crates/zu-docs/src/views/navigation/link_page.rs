// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, Html};
use zu::link::{Link, Underline};
use zu::styles::color::Color;
use zu::typography::Variant;

use crate::components::demo_box::DemoBox;

#[function_component(LinkPage)]
pub fn link_page() -> Html {
    html! {
        <div class="container">
        <h1>{"Links"}</h1>

        <h2>{"Basic links"}</h2>
        <p>{"The Link component is built on top of the Typography component, meaning that you can use its props."}</p>
        <DemoBox>
            <Link href="#">{"Link"}</Link>
            <Link href="#" color={Color::Inherit}>{ "color='inherit'"}</Link>
            <Link href="#" variant={Variant::Body2}>{"variant='body2'"}</Link>
        </DemoBox>
        <div>
            <p>{"However, the Link component has some different default props than the Typography component:"}</p>
        <ul>
        <li><code>{"color='primary'"}</code>{" as the link needs to stand out."}</li>
        <li><code>{"variant='inherit'"}</code>{" as the link will, most of the time, be used as a child
            of a Typography component."}</li>
        </ul>
        </div>

        <h2>{"Underline"}</h2>
        <p>{"The "}<code>{"underline"}</code> {"prop can be used to set the underline behavior.
            The default is always."}</p>
        <DemoBox>
            <Link href="#" underline={Underline::None}>{"underline='none'"}</Link>
            <Link href="#" underline={Underline::Hover}>{"underline='hover'"}</Link>
            <Link href="#" underline={Underline::Always}>{"underline='always'"}</Link>
        </DemoBox>

        <h2>{"Security"}</h2>
        <p>{"When you use"} <code>{"target=\"_blank\""}</code> {" with Links, it is recommended to always set "}
        <code>{"rel=\"noopener\""}</code> {" or "} <code>{"rel=\"noreferrer\""}</code>
        {" when linking to third party content."}</p>
        <ul>
        <li><code>{"rel=\"noopener\""}</code> {" prevents the new page from being able to access the window.opener
            property and ensures it runs in a separate process. Without this, the target page
            can potentially redirect your page to a malicious URL."}</li>
        <li><code>{"rel=\"noreferrer\""}</code> {" has the same effect, but also prevents the Referer header
            from being sent to the new page. Removing the referrer header will affect analytics."}</li>
        </ul>
        </div>
    }
}
