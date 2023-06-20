// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::Routable;

use crate::views::alert_page::AlertPage;
use crate::views::autocomplete_page::AutocompletePage;
use crate::views::avatar_page::AvatarPage;
use crate::views::backdrop_page::BackdropPage;
use crate::views::badge_page::BadgePage;
use crate::views::bottom_navigation_page::BottomNavigationPage;
use crate::views::box_page::BoxPage;
use crate::views::breadcrumbs_page::BreadcrumbsPage;
use crate::views::button_group_page::ButtonGroupPage;
use crate::views::button_page::ButtonPage;
use crate::views::card_page::CardPage;
use crate::views::checkbox_page::CheckboxPage;
use crate::views::chip_page::ChipPage;
use crate::views::container_page::ContainerPage;
use crate::views::divider_page::DividerPage;
use crate::views::floating_action_button_page::FloatingActionButtonPage;
use crate::views::home_page::HomePage;
use crate::views::icons_page::IconsPage;
use crate::views::input_page::InputPage;
use crate::views::material_icons_page::MaterialIconsPage;
use crate::views::paper_page::PaperPage;
use crate::views::progress_page::ProgressPage;
use crate::views::radio_group_page::RadioGroupPage;
use crate::views::skeleton_page::SkeletonPage;
use crate::views::stack_page::StackPage;
use crate::views::switch_page::SwitchPage;
use crate::views::typography_page::TypographyPage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/alert")]
    Alert,

    #[at("/autocomplete")]
    Autocomplete,

    #[at("/avatar")]
    Avatar,

    #[at("/backdrop")]
    Backdrop,

    #[at("/badge")]
    Badge,

    #[at("/bottom-navigation")]
    BottomNavigation,

    #[at("/box")]
    Box,

    #[at("/breadcrumbs")]
    Breadcrumbs,

    #[at("/button")]
    Button,

    #[at("/button-group")]
    ButtonGroup,

    #[at("/card")]
    Card,

    #[at("/checkbox")]
    Checkbox,

    #[at("/chip")]
    Chip,

    #[at("/container")]
    Container,

    #[at("/divider")]
    Divider,

    #[at("/floating-action-button")]
    FloatingActionButton,

    #[at("/material-icons")]
    MaterialIcons,

    #[at("/icons")]
    Icons,

    #[at("/input")]
    Input,

    #[at("/paper")]
    Paper,

    #[at("/progress")]
    Progress,

    #[at("/radio-group")]
    RadioGroup,

    #[at("/skeleton")]
    Skeleton,

    #[at("/stack")]
    Stack,

    #[at("/switch")]
    Switch,

    #[at("/typography")]
    Typography,
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
        Route::Input => html! {<InputPage />},
        Route::MaterialIcons => html! { <MaterialIconsPage />},
        Route::Paper => html! {<PaperPage />},
        Route::Progress => html! {<ProgressPage />},
        Route::RadioGroup => html! {<RadioGroupPage />},
        Route::Skeleton => html! {<SkeletonPage />},
        Route::Stack => html! {<StackPage />},
        Route::Switch => html! {<SwitchPage />},
        Route::Typography => html! {<TypographyPage />},
    }
}
