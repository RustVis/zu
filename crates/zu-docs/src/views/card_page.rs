// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::avatar::Avatar;
use zu::button::{Button, Size};
use zu::card::Card;
use zu::card_actions::CardActions;
use zu::card_content::CardContent;
use zu::card_header::CardHeader;
use zu::card_media::CardMedia;
use zu::paper::Variant as PaperVariant;
use zu::styles::color::Color;
use zu::typography::{Typography, Variant as TypographyVariant};
use zuicon_material::{Favorite, MoreVert, Share};

use crate::components::demo_box::DemoBox;

#[function_component(CardPage)]
pub fn card_page() -> Html {
    html! {
        <div class="container">
        <h1>{"Card"}</h1>
        <p>{"Cards are surfaces that display content and actions on a single topic."}</p>

        <h2>{"Basic Card"}</h2>
        <p>{"Although cards can support multiple actions, UI controls, and an overflow menu, \
        use restraint and remember that cards are entry points to more complex and detailed information."}</p>
        <DemoBox>
            <Card style="min-width: 275px;">
            <CardContent>
                <Typography style="font-size: 14px"
                    color={Color::Secondary}
                    gutter_bottom={true}>
                    {"Word of the Day"}
                </Typography>
                <Typography variant={TypographyVariant::H5} component="div">
                    {"be{bull}nev{bull}o{bull}lent"}
                </Typography>
                <Typography style="margin-bottom: 1.5em;" color={Color::Secondary}>
                    {"adjective"}
                </Typography>
                <Typography variant={TypographyVariant::Body2}>
                    {"well meaning and kindly."}
                    <br />
                    {"a benevolent smile"}
                </Typography>
            </CardContent>
            <CardActions>
                <Button size={Size::Small}>{"Learn More"}</Button>
            </CardActions>
            </Card>
        </DemoBox>

        <h2>{"Outlined Card"}</h2>
        <p>{"Set variant=outlined to render an outlined card."}</p>
        <DemoBox>
            <Card style="min-width: 275px;" variant={PaperVariant::Outlined}>
            <CardContent>
                <Typography style="font-size: 14px"
                    color={Color::Secondary}
                    gutter_bottom={true}>
                    {"Word of the Day"}
                </Typography>
                <Typography variant={TypographyVariant::H5} component="div">
                    {"be{bull}nev{bull}o{bull}lent"}
                </Typography>
                <Typography style="margin-bottom: 1.5em;" color={Color::Secondary}>
                    {"adjective"}
                </Typography>
                <Typography variant={TypographyVariant::Body2}>
                    {"well meaning and kindly."}
                    <br />
                    {"a benevolent smile"}
                </Typography>
            </CardContent>
            <CardActions>
                <Button size={Size::Small}>{"Learn More"}</Button>
            </CardActions>
            </Card>
        </DemoBox>

        <h2>{"Complex Interaction"}</h2>
        <p>{"On desktop, card content can expand. (Click the downward chevron to view the recipe.)"}</p>
        <DemoBox>
        <Card style="max-width: 345px;">
        <CardHeader
            avatar={html!{
                <Avatar style="background-color: var(--zu-colors-red-500);" aria_label="recipe">{"R"}</Avatar>
            }}
            action={html!{<MoreVert />}}
            title={html!{"Shrimp and Chorizo Paella"}}
            subheader={html!{"September 14, 2016"}}
            />
        <CardMedia
            component="img"
            height={194}
            image="/static/images/cards/paella.jpg"
            alt="Paella dish"
            />
        <CardContent>
            <Typography variant={TypographyVariant::Body2} color={Color::Secondary}>
                {"This impressive paella is a perfect party dish and a fun meal to cook \
                together with your guests. Add 1 cup of frozen peas along with the mussels, \
                if you like."}
            </Typography>
        </CardContent>
        <CardActions disable_spacing={true}>
            <Favorite />
            <Share />
        </CardActions>
        <CardContent>
            <Typography paragraph={true}>{"Method:"}</Typography>
            <Typography paragraph={true}>
                {"Heat 1/2 cup of the broth in a pot until simmering, add saffron and set
                aside for 10 minutes."}
            </Typography>
            <Typography paragraph={true}>
                {"Heat oil in a (14- to 16-inch) paella pan or a large, deep skillet over
                medium-high heat. Add chicken, shrimp and chorizo, and cook, stirring
                occasionally until lightly browned, 6 to 8 minutes. Transfer shrimp to a
                large plate and set aside, leaving chicken and chorizo in the pan. Add
                pimentón, bay leaves, garlic, tomatoes, onion, salt and pepper, and cook,
                stirring often until thickened and fragrant, about 10 minutes. Add
                saffron broth and remaining 4 1/2 cups chicken broth; bring to a boil."}
            </Typography>
            <Typography paragraph={true}>
                {"Add rice and stir very gently to distribute. Top with artichokes and
                peppers, and cook without stirring, until most of the liquid is absorbed,
                15 to 18 minutes. Reduce heat to medium-low, add reserved shrimp and
                mussels, tucking them down into the rice, and cook again without
                stirring, until mussels have opened and rice is just tender, 5 to 7
                minutes more. (Discard any mussels that don&apos;t open.)"}
            </Typography>
            <Typography>
                {"Set aside off of the heat to let rest for 10 minutes, and then serve."}
            </Typography>
        </CardContent>
        </Card>
        </DemoBox>

        <h2>{"Media"}</h2>
        <p>{"Example of a card using an image to reinforce the content."}</p>
        <DemoBox>
        <Card style="max-width: 345px">
            <CardMedia style="height: 140px"
                image="/assets/images/cards/contemplative-reptile.jpg"
                title="green iguana"/>
            <CardContent>
                <Typography gutter_bottom={true} variant={TypographyVariant::H5} component="div">
                    {"Lizard"}
                </Typography>
                <Typography variant={TypographyVariant::Body2} color={Color::Secondary}>
                    {"Lizards are a widespread group of squamate reptiles, with over 6,000
                    species, ranging across all continents except Antarctica"}
                </Typography>
            </CardContent>
            <CardActions>
                <Button size={Size::Small}>{"Share"}</Button>
                <Button size={Size::Small}>{"Learn More"}</Button>
            </CardActions>
        </Card>
        </DemoBox>

        </div>
    }
}
