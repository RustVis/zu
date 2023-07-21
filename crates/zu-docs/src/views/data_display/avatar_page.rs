// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Html};
use zu::avatar::{Avatar, Props};
use zu::avatar_group::AvatarGroup;
use zu::badge::{
    Badge, Content as BadgeContent, Overlap as BadgeOverlap, Props as BadgeProps,
    Variant as BadgeVariant,
};
use zu::styles::anchor_origin::AnchorOrigin;
use zu::styles::shape_variant::ShapeVariant;
use zuicon_material::{Assignment, Folder, Pageview};

use crate::components::demo_box::DemoBox;

#[function_component(StyledBadge)]
pub fn styled_badge(props: &BadgeProps) -> Html {
    let style_content = include_str!("avatar_page_styled_badge.css");
    let style = stylist::Style::new(style_content)
        .expect("Invalid style file: avatar_page_styled_badge.css");
    let new_props = BadgeProps {
        badge_classes: classes!(style.get_class_name().to_owned()),
        ..props.clone()
    };
    html! {
        <Badge ..new_props />
    }
}

#[function_component(SmallAvatar)]
pub fn small_avatar(props: &Props) -> Html {
    let style = "width: 22px;
        height: 22px;
        border: 2px solid var(--zu-palette-background-paper);";
    let new_props = Props {
        style: AttrValue::from(style),
        ..props.clone()
    };
    html! {
        <Avatar ..new_props />
    }
}

fn create_image_avatars_view() -> Html {
    html! {
        <>
        <h2>{"Image avatars"}</h2>
        <p>{"Image avatars can be created by passing standard img props src or srcSet to the component."}</p>
        <DemoBox>
            <Avatar alt="Remy Sharp" src="/images/avatar/1.jpg" />
            <Avatar alt="Travis Howard" src="/images/avatar/2.jpg" />
            <Avatar alt="Cindy Baker" src="/images/avatar/3.jpg" />
        </DemoBox>
        </>
    }
}

fn create_letter_avatars_view() -> Html {
    html! {
        <>
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
        </>
    }
}

fn create_size_view() -> Html {
    html! {
        <>
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
        </>
    }
}

fn create_icon_avatars_view() -> Html {
    html! {
        <>
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
        </>
    }
}

fn create_variants_view() -> Html {
    html! {
        <>
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
        </>
    }
}

fn create_fallbacks_view() -> Html {
    html! {
        <>
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
        </>
    }
}

fn create_grouped_view() -> Html {
    html! {
        <>
        <h2>{"Grouped"}</h2>
        <p>{"AvatarGroup renders its children as a stack. Use the max prop to limit the number of avatars."}</p>
        <DemoBox>
            <AvatarGroup max={4}>
                <Avatar alt="Remy Sharp" src="/images/avatar/1.jpg" />
                <Avatar alt="Travis Howard" src="/images/avatar/2.jpg" />
                <Avatar alt="Cindy Baker" src="/images/avatar/3.jpg" />
                <Avatar alt="Agnes Walker" src="/images/avatar/4.jpg" />
                <Avatar alt="Trevor Henderson" src="/images/avatar/5.jpg" />
            </AvatarGroup>
        </DemoBox>

        <h3>{"Total avatars"}</h3>
        <p>{"If you need to control the total number of avatars not shown, you can use the total prop."}</p>
        <DemoBox>
            <AvatarGroup total={24}>
                <Avatar alt="Remy Sharp" src="/images/avatar/1.jpg" />
                <Avatar alt="Travis Howard" src="/images/avatar/2.jpg" />
                <Avatar alt="Cindy Baker" src="/images/avatar/3.jpg" />
                <Avatar alt="Trevor Henderson" src="/images/avatar/5.jpg" />
            </AvatarGroup>
        </DemoBox>
        </>
    }
}

fn create_with_badge_view() -> Html {
    html! {
        <>
        <h2>{"With badge"}</h2>
        <p>{""}</p>
        <DemoBox>
            <StyledBadge
                overlap={BadgeOverlap::Circular}
                anchor_origin={AnchorOrigin::bottom_right()}
                variant={BadgeVariant::Dot}
                >
                <Avatar alt="Remy Sharp" src="/images/avatar/1.jpg" />
            </StyledBadge>
            <Badge
                overlap={BadgeOverlap::Circular}
                anchor_origin={AnchorOrigin::bottom_right()}
                content={
                    BadgeContent::Node(
                    html!{<SmallAvatar alt="Remy Sharp" src="/images/avatar/1.jpg" />}
                )}>
                <Avatar alt="Travis Howard" src="/images/avatar/2.jpg" />
            </Badge>
        </DemoBox>
        </>
    }
}

#[function_component(AvatarPage)]
pub fn avatar_page() -> Html {
    html! {
        <div class="container">
        <h1>{"Avatar"}</h1>

        {create_image_avatars_view()}
        {create_letter_avatars_view()}
        {create_size_view()}
        {create_icon_avatars_view()}
        {create_variants_view()}
        {create_fallbacks_view()}
        {create_grouped_view()}
        {create_with_badge_view()}

        </div>
    }
}
