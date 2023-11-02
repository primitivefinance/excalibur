//! An example screen that deploys the Counter.sol smart contract.
//! Screens are individual views that can be rendered by the application.
//! The application also handles the execution of the screen's messages and
//! events.

use std::sync::Arc;

use arbiter_core::middleware::RevmMiddleware;
use iced::{
    alignment::{self},
    widget::{button, column, text},
    Element, Length,
};
use thiserror::Error;

use crate::sdk::vault::*;

use super::watcher;

mod banner;
pub mod start;

/// Screen for the deploy Counter.sol example.
#[derive(Debug, Clone)]
pub struct ExampleScreen {
    pub client: Arc<RevmMiddleware>,
    pub state: ExampleScreenState,
    pub watcher: watcher::WatcherComponent,
}

/// States of the example screen.
/// This example is only concerned with the deployed state of Counter.sol
#[derive(Debug, Clone)]
pub enum ExampleScreenState {
    NotDeployed,
    Deployed(Vault),
    DeploymentFailed(ExampleScreenError),
}

/// Messages for Application -> Screen communication.
#[derive(Clone, Debug)]
pub enum ExampleScreenMessage {
    Deploy,
    DeploySuccess(Result<Vault, ExampleScreenError>),
    Empty,
    WatcherComponent(watcher::AppToWatcherMessage),
}

/// Errors that can occur during the deployment of Counter.sol.
#[derive(Debug, Error, Clone)]
pub enum ExampleScreenError {
    #[error("API Error")]
    ProviderError(#[from] &'static ethers::providers::ProviderError),
}

/// Messages for Screen -> Application communication.
#[derive(Clone)]
pub enum Event {
    Clicked,
    Deployed(Result<Vault, ExampleScreenError>),
    Toggle(bool),
}

/// Implements the following functions
/// - new - creates a new instance of the screen
/// - update - handles messages and returns events, called by the Application.
/// - view - renders the screen, called by the Application.
impl ExampleScreen {
    pub fn new(client: Arc<RevmMiddleware>) -> Self {
        Self {
            client,
            state: ExampleScreenState::NotDeployed,
            watcher: watcher::WatcherComponent::new(),
        }
    }

    /// This logic can be a little confusing at first, here's a summary of
    /// what's going on:
    /// - The application renders this screen in `ExampleApp::view()`.
    /// - This screen has a button that triggers a message, i.e.
    ///   `ExampleScreenMessage::Deploy`.
    /// - The `ExampleApp::view()` function captures the triggered message in
    ///   the screen's `view()`.
    /// - Captured messages get passed to `ExampleApp::update()`.
    /// - `ExampleApp::update()` calls `ExampleScreen::update()` with the
    ///   captured message.
    /// - `ExampleScreen::update()` returns an event.
    /// - `ExampleApp` matches the event with a command or message to perform.
    /// - `ExampleApp` executes a `perform` or updates the screen's state.
    ///
    /// This design will probably be updated, but right now we are using it
    /// because only the `Application` can execute commands that are async.
    pub fn update(&mut self, message: ExampleScreenMessage) -> Option<Event> {
        match message {
            ExampleScreenMessage::WatcherComponent(message) => {
                // Call the update method to get the component's updates.
                let watcher_message = self.watcher.update(message);

                match watcher_message {
                    Some(watcher_message) => {
                        // If the watcher has a message, return it.
                        match watcher_message {
                            watcher::WatcherToAppMessage::Toggle(state) => {
                                Some(Event::Toggle(state))
                            }
                        }
                    }
                    None => None,
                }
            }
            ExampleScreenMessage::Empty => None,
            ExampleScreenMessage::Deploy => Some(Event::Clicked),
            ExampleScreenMessage::DeploySuccess(res) => Some(Event::Deployed(res)),
        }
    }

    pub fn view<'a>(&self) -> Element<'a, ExampleScreenMessage> {
        let mut content = column![];

        let button = |label, on_press| {
            button(
                text(label)
                    .width(Length::Fill)
                    .height(40)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .on_press(on_press)
        };
        content = content.push(button("Deploy Counter.sol", ExampleScreenMessage::Deploy));

        content = match &self.state {
            ExampleScreenState::NotDeployed => content,
            ExampleScreenState::Deployed(_entity) => content.push(text("Successfully Deployed")),
            ExampleScreenState::DeploymentFailed(error) => {
                content.push(text(format!("Deployment failed: {:?}", error)))
            }
        };

        // Render Watcher component
        content = content.push(
            self.watcher
                .view()
                .map(|message| ExampleScreenMessage::WatcherComponent(message)),
        );

        content.spacing(4).into()
    }
}
