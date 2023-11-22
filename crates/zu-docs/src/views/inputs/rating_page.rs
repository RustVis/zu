// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use stylist::Style;
use yew::{classes, function_component, html, Html};
use zu::r#box::Box;
use zu::rating::{Props as RatingProps, Rating};
use zu::stack::Stack;
use zu::styles::size::Size;
use zu::styles::spacing::Spacing;
use zu::svg_icon::FontSize;
use zu::typography::Typography;
use zu_util::cmp::fuzzy_compare;
use zuicon_material::{Favorite, FavoriteBorder, Star};

use crate::components::demo_box::DemoBox;

fn create_basic_rating_view() -> Html {
    let value = 2.0;
    let box_style = Style::new(
        r"
        & > legend {
            margin-top: 12px;
        }
    ",
    )
    .expect("Failed to parse box style");

    html! {
        <>
        <h2>{"Basic rating"}</h2>
        <DemoBox>
        <Box classes={classes!(box_style.get_class_name().to_owned())}>
            <Typography component="legend">{"Controlled"}</Typography>
            <Rating name="simple-controlled" value={Some(value)} />

            <Typography component="legend">{"Read only"}</Typography>
            <Rating name="read-only" value={Some(value)} read_only=true />

            <Typography component="legend">{"Disabled"}</Typography>
            <Rating name="disabled" value={Some(value)} disabled=true />

            <Typography component="legend">{"No rating given"}</Typography>
            <Rating name="no-value" value={None} />
        </Box>
        </DemoBox>
        </>
    }
}

fn create_rating_precision_view() -> Html {
    html! {
        <>
        <h2>{"Rating precision"}</h2>
        <p>{"The rating can display any float number with the value prop. Use the precision prop \
        to define the minimum increment value change allowed."}</p>

        <DemoBox>
        <Stack spacing={Spacing::Small}>
            <Rating name="half-rating" default_value=2.5 precision=0.5 />
            <Rating name="half-rating-read" default_value=2.5 precision=0.5 read_only=true />
        </Stack>
        </DemoBox>
        </>
    }
}

fn create_hover_feedback_view() -> Html {
    let labels = &[
        (0.5, "Useless"),
        (1.0, "Useless+"),
        (1.5, "Poor"),
        (2.0, "Poor+"),
        (2.5, "Ok"),
        (3.0, "Ok+"),
        (3.5, "Good"),
        (4.0, "Good+"),
        (4.5, "Excellent"),
        (5.0, "Excellent+"),
    ];
    let value = 2.5;
    let get_label = |index| -> &'static str {
        for (key, value) in labels {
            if fuzzy_compare(*key, index) {
                return value;
            }
        }
        ""
    };

    html! {
        <>
        <h2>{"Hover feedback"}</h2>
        <p>{"You can display a label on hover to help the user pick the correct rating value. \
        The demo uses the onChangeActive prop."}</p>

        <DemoBox>
        <Box style="width: 200px; display: flex; align-items: center">
            <Rating name="hover-feedback"
                value={value}
                precision=0.5
                empty_icon={html!{<Star style="opacity: 0.55;" font_size={FontSize::Inherit} />}}
            />
            <Box style="margin-left: 12px;">{get_label(value)}</Box>
        </Box>
        </DemoBox>
        </>
    }
}

fn create_sizes_view() -> Html {
    html! {
        <>
        <h2>{"Sizes"}</h2>
        <p>{"For larger or smaller ratings use the size prop."}</p>
        <DemoBox>
        <Stack spacing={Spacing::Small}>
            <Rating name="size-small" default_value=2.0 size={Size::Small} />
            <Rating name="size-medium" default_value=2.0 />
            <Rating name="size-large" default_value=2.0 size={Size::Large} />
        </Stack>
        </DemoBox>
        </>
    }
}

#[function_component(StyledRating)]
pub fn styled_rating(props: &RatingProps) -> Html {
    let new_style = Style::new(
        r"
    & .ZuRating-iconFilled {
        color: #ff6d75;
    }
    
    & .ZuRating-iconHover {
        color: #ff3d47;
    }
    ",
    )
    .expect("Failed to parse new style of StyledRating");
    let new_props = RatingProps {
        classes: classes!(new_style.get_class_name().to_owned()),
        ..props.clone()
    };

    html! {
        <Rating ..new_props />
    }
}

fn create_customization_view() -> Html {
    let box_style = Style::new(
        r"
        & > legend {
            margin-top: 12px;
        }
    ",
    )
    .expect("Failed to parse box style");

    html! {
        <>
        <h2>{"Customization"}</h2>
        <p>{"Here are some examples of customizing the component. \
        You can learn more about this in the overrides documentation page."}</p>

        <DemoBox>
        <Box classes={classes!(box_style.get_class_name().to_owned())}>
            <Typography component="legend">{"Custom icon and color"}</Typography>
            <StyledRating
                name="customized-color"
                default_value=2.0
                precision=0.5
                icon={html!{<Favorite font_size={FontSize::Inherit} />}}
                empty_icon={html!{<FavoriteBorder font_size={FontSize::Inherit} />}}
                />

            <Typography component="legend">{"10 stars"}</Typography>
            <Rating name="customized-10" default_value=2.0 max=10.0 />
        </Box>
        </DemoBox>
        </>
    }
}

#[function_component(RadioStyledRating)]
pub fn radio_styled_rating(props: &RatingProps) -> Html {
    let new_style = Style::new(
        r"
    & .ZuRating-iconEmpty .ZuSvgIcon-root {
      color: var(--zu-palette-action-disabled);
    }
    ",
    )
    .expect("Failed to parse new style of RadioStyledRating");
    let new_props = RatingProps {
        classes: classes!(new_style.get_class_name().to_owned()),
        ..props.clone()
    };

    html! {
        <Rating ..new_props />
    }
}

fn create_radio_group_view() -> Html {
    html! {
        <>
        <h2>{"Radio group"}</h2>
        <p>{"The rating is implemented with a radio group, set highlightSelectedOnly \
        to restore the natural behavior."}</p>
        <DemoBox>
        <RadioStyledRating
            name="highlight-selected-only"
            default_value=2.0
            highlight_selected_only={true}
            />
        </DemoBox>
        </>
    }
}

#[function_component(RatingPage)]
pub fn rating_page() -> Html {
    html! {
        <div class="container">
        <h1>{"Rating"}</h1>

        {create_basic_rating_view()}
        {create_rating_precision_view()}
        {create_hover_feedback_view()}
        {create_sizes_view()}
        {create_customization_view()}
        {create_radio_group_view()}

        </div>
    }
}
