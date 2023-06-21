// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::Routable;

use crate::views::data_display::{
    AvatarPage, BadgePage, ChipPage, DividerPage, IconsPage, MaterialIconsPage, TypographyPage,
};
use crate::views::feedback::{AlertPage, BackdropPage, ProgressPage, SkeletonPage};
use crate::views::home_page::HomePage;
use crate::views::inputs::{
    AutocompletePage, ButtonGroupPage, ButtonPage, CheckboxPage, FloatingActionButtonPage,
    RadioGroupPage, SwitchPage,
};
use crate::views::layout::{BoxPage, ContainerPage, StackPage};
use crate::views::navigation::{BottomNavigationPage, BreadcrumbsPage};
use crate::views::surfaces::{CardPage, PaperPage};

use crate::views::inputs::RatingPage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,

    // inputs
    #[at("/autocomplete")]
    Autocomplete,
    #[at("/button")]
    Button,
    #[at("/button-group")]
    ButtonGroup,
    #[at("/checkbox")]
    Checkbox,
    #[at("/floating-action-button")]
    FloatingActionButton,
    #[at("/radio-group")]
    RadioGroup,
    #[at("/rating")]
    Rating,
    #[at("/switch")]
    Switch,

    // data display
    #[at("/avatar")]
    Avatar,
    #[at("/badge")]
    Badge,
    #[at("/chip")]
    Chip,
    #[at("/divider")]
    Divider,
    #[at("/icons")]
    Icons,
    #[at("/material-icons")]
    MaterialIcons,
    #[at("/typography")]
    Typography,

    // feedback
    #[at("/alert")]
    Alert,
    #[at("/backdrop")]
    Backdrop,
    #[at("/progress")]
    Progress,
    #[at("/skeleton")]
    Skeleton,

    // surfaces
    #[at("/card")]
    Card,
    #[at("/paper")]
    Paper,

    // navigation
    #[at("/bottom-navigation")]
    BottomNavigation,
    #[at("/breadcrumbs")]
    Breadcrumbs,

    // layout
    #[at("/box")]
    Box,
    #[at("/container")]
    Container,
    #[at("/stack")]
    Stack,
    // utils
}

#[must_use]
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::let_unit_value)]
pub fn switch_route(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<HomePage />},
        Route::Alert => html! {<AlertPage />},
        Route::Autocomplete => html! {<AutocompletePage />},
        Route::Avatar => html! {<AvatarPage />},
        Route::Backdrop => html! {<BackdropPage />},
        Route::Badge => html! {<BadgePage />},
        Route::BottomNavigation => html! {<BottomNavigationPage />},
        Route::Box => html! {<BoxPage />},
        Route::Breadcrumbs => html! {<BreadcrumbsPage />},
        Route::Button => html! {<ButtonPage />},
        Route::ButtonGroup => html! {<ButtonGroupPage />},
        Route::Card => html! {<CardPage />},
        Route::Checkbox => html! {<CheckboxPage />},
        Route::Chip => html! {<ChipPage />},
        Route::Container => html! {<ContainerPage />},
        Route::Divider => html! {<DividerPage />},
        Route::FloatingActionButton => html! {<FloatingActionButtonPage />},
        Route::Icons => html! {<IconsPage />},
        Route::MaterialIcons => html! { <MaterialIconsPage />},
        Route::Paper => html! {<PaperPage />},
        Route::Progress => html! {<ProgressPage />},
        Route::RadioGroup => html! {<RadioGroupPage />},
        Route::Rating => html! {<RatingPage />},
        Route::Skeleton => html! {<SkeletonPage />},
        Route::Stack => html! {<StackPage />},
        Route::Switch => html! {<SwitchPage />},
        Route::Typography => html! {<TypographyPage />},
    }
}
