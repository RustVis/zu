// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::avatar::Avatar;
use zu::avatar_group::AvatarGroup;
use zu::styles::shape_variant::ShapeVariant;
use zuicon_material::{Assignment, Folder, Pageview};

use crate::components::demo_box::DemoBox;

#[function_component(AvatarPage)]
pub fn avatar_page() -> Html {
    // TODO(Shaohua): Support badge

    html! {
        <div class="container">
        <h1>{"Avatar"}</h1>
        <h2>{"Image avatars"}</h2>
        <p>{"Image avatars can be created by passing standard img props src or srcSet to the component."}</p>
        <DemoBox>
            <Avatar alt="Remy Sharp" src="/images/avatar/1.jpg" />
            <Avatar alt="Travis Howard" src="/images/avatar/2.jpg" />
            <Avatar alt="Cindy Baker" src="/images/avatar/3.jpg" />
        </DemoBox>

        <h2>{"Letter avatars"}</h2>
        <p>{"Avatars containing simple characters can be created by passing a string as children."}</p>
        <DemoBox>
            <Avatar>{"H"}</Avatar>
            <Avatar style="background-color: var(--zu-colors-deepOrange-500);">{"N"}</Avatar>
            <Avatar style="background-color: var(--zu-colors-deepPurple-500);">{"OP"}</Avatar>
        </DemoBox>

        <p>{"You can use different background colors for the avatar.\
         The following demo generates the color based on the name of the person."}</p>
        <DemoBox>
            <Avatar alt="Kent Dodds" />
            <Avatar alt="Jed Watson" />
            <Avatar alt="Tim Neutkens" />
        </DemoBox>

        <h2>{"Size"}</h2>
        <p>{"You can change the size of the avatar with the height and width CSS properties."}</p>
        <DemoBox>
            <Avatar alt="Remy Sharp"
                src="/images/avatar/1.jpg"
                style="width: 24px; height: 24px;"
            />
            <Avatar alt="Remy Sharp" src="/images/avatar/1.jpg" />
            <Avatar alt="Remy Sharp" src="/images/avatar/1.jpg"
                style="width: 56px; height: 56px;"
            />
        </DemoBox>

        <h2>{"Icon avatars"}</h2>
        <p>{"Icon avatars are created by passing an icon as children."}</p>
        <DemoBox>
            <Avatar>
                <Folder />
            </Avatar>
            <Avatar style="background-color: var(--zu-colors-pink-500);">
                <Pageview />
            </Avatar>
            <Avatar style="background-color: var(--zu-colors-green-500);">
                <Assignment />
            </Avatar>
        </DemoBox>

        <h2>{"Variants"}</h2>
        <p>{"If you need square or rounded avatars, use the variant prop."}</p>
        <DemoBox>
            <Avatar style="background-color: var(--zu-colors-deepOrange-500);"
                variant={ShapeVariant::Square}>{"N"}</Avatar>
            <Avatar style="background-color: var(--zu-colors-green-500)"
                variant={ShapeVariant::Rounded}>
                <Assignment />
            </Avatar>
        </DemoBox>

        <h2>{"Fallbacks"}</h2>
        <p>{"If there is an error loading the avatar image, the component falls back to an alternative"}</p>
        <DemoBox>
            <Avatar style="background-color: var(--zu-colors-deepOrange-500);"
                alt="Remy Sharp"
                src="/images/broken-image.jpg">
                {"B"}
            </Avatar>
            <Avatar style="background-color: var(--zu-colors-deepOrange-500);"
                alt="Remy Sharp"
                src="/images/broken-image.jpg"
                />
            <Avatar src="/images/broken-image.jpg" />
        </DemoBox>

        <h2>{"Grouped"}</h2>
        <p>{"AvatarGroup renders its children as a stack. Use the max prop to limit the number of avatars."}</p>
        <DemoBox>
            <AvatarGroup max={4}>
                <Avatar alt="Remy Sharp" src="/images/avatar/1.jpg" />
                <Avatar alt="Travis Howard" src="/images/avatar/2.jpg" />
                <Avatar alt="Cindy Baker" src="/static/images/avatar/3.jpg" />
                <Avatar alt="Agnes Walker" src="/static/images/avatar/4.jpg" />
                <Avatar alt="Trevor Henderson" src="/static/images/avatar/5.jpg" />
            </AvatarGroup>
        </DemoBox>

        <h2>{"With badge"}</h2>
        </div>
    }
}
