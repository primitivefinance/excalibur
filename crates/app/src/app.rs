use std::collections::VecDeque;

use tracer::AppEventLog;
use tracing::Span;
use user::{
    contacts::{self, ContactValue},
    UserProfile,
};

use super::{
    screens::{empty::EmptyScreen, exit::ExitScreen, Screen},
    *,
};
use crate::{
    middleware::ExcaliburMiddleware,
    screens::{
        dev::experimental::ExperimentalScreen, portfolio::PortfolioRoot, settings::SettingsScreen,
        State,
    },
    user::networks::RPCValue,
    view::sidebar::Sidebar,
};

pub fn app_span() -> Span {
    tracing::debug_span!("App")
}

/// Root message for the Application.
#[derive(Debug, Default)]
pub enum Message {
    #[default]
    Empty,
    Load,
    View(view::Message),
    CacheMessage(CacheMessage),
    StorageMessage(StorageMessage),
    WindowsMessage(WindowsMessage),
    Exit,
}

pub type RootMessage = Message;
pub type RootViewMessage = view::Message;

impl MessageWrapper for Message {
    type ParentMessage = Message;
}

/// State for all temporarily cached state.
#[derive(Default)]
pub struct Cache {
    pub app_events: VecDeque<AppEventLog>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            app_events: VecDeque::new(),
        }
    }
}

#[derive(Debug)]
pub enum CacheMessage {
    AppEvent(AppEventLog),
}

/// State for all permanent state that is loaded from disk or api.
#[derive(Debug, Clone, Default)]
pub struct Storage {
    pub profile: UserProfile,
}

#[derive(Debug)]
pub enum StorageMessage {
    AddressBook(AddressBookMessage),
    RPCStorage(RPCStorageMessage),
    AnvilSnapshot(anyhow::Result<String>),
}

#[derive(Debug)]
pub enum AddressBookMessage {
    Add(String, Address, contacts::Category),
    Remove(String, contacts::Category),
    Get(String, contacts::Category),
    List(contacts::Category),
    Clear(contacts::Category),
}

#[derive(Debug)]
pub enum RPCStorageMessage {
    Add(RPCValue),
    Remove(String),
    Get(String),
    List,
    Clear,
}

impl From<RPCStorageMessage> for StorageMessage {
    fn from(msg: RPCStorageMessage) -> Self {
        Self::RPCStorage(msg)
    }
}

impl From<StorageMessage> for Message {
    fn from(msg: StorageMessage) -> Self {
        Self::StorageMessage(msg)
    }
}

impl From<RPCStorageMessage> for Message {
    fn from(msg: RPCStorageMessage) -> Self {
        Self::StorageMessage(msg.into())
    }
}

/// State for specific windows that are open.
pub struct Windows {
    pub screen: Screen,
    pub sidebar: Sidebar,
}

impl Default for Windows {
    fn default() -> Self {
        Self {
            screen: ExperimentalScreen::new().into(),
            sidebar: Sidebar::new(),
        }
    }
}

impl Windows {
    pub fn new(screen: Screen, sidebar: Sidebar) -> Self {
        Self { screen, sidebar }
    }
}

#[derive(Debug)]
pub enum WindowsMessage {
    Switch(view::sidebar::Route),
}

impl From<WindowsMessage> for Message {
    fn from(msg: WindowsMessage) -> Self {
        Message::WindowsMessage(msg)
    }
}

/// Storage for the entire application.
/// This should hold the most important pieces of data that many children
/// components will need.
pub struct App {
    /// Connection to networks.
    pub client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
    /// Transient state for the application.
    pub cache: Cache,
    /// Persistent state for the application.
    pub storage: Storage,
    /// State of the active window and sidebar the user is viewing.
    pub windows: Windows,
}

impl App {
    pub fn new(
        storage: Storage,
        client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
    ) -> (Self, Command<Message>) {
        let dashboard = PortfolioRoot::new(Some(client.clone()), storage.profile.clone()).into();
        (
            Self {
                client,
                storage,
                cache: Cache::new(),
                windows: Windows::new(dashboard, Sidebar::new()),
            },
            Command::perform(async {}, |_| Message::Load),
        )
    }

    // Loads the sidebar and the default screen.
    pub fn load(&mut self) -> Command<Message> {
        let mut cmds = Vec::new();

        // Load the sidebar.
        cmds.push(self.windows.sidebar.load().map(|x| x.into()));

        // Load the current window.
        cmds.push(self.windows.screen.load().map(|x| x.into()));

        Command::batch(cmds)
    }

    // All view updates are forwarded to the Screen's update function.
    pub fn update(&mut self, message: Message) -> Command<Message> {
        app_span().in_scope(|| match message {
            Message::Exit => iced::window::close(),
            Message::Load => self.load(),
            Message::StorageMessage(msg) => self.storage_update(msg),
            Message::CacheMessage(msg) => self.cache_update(msg),
            Message::WindowsMessage(msg) => self.windows_update(msg),
            Message::View(view::Message::Route(route)) => self.switch_window(&route),
            Message::View(view::Message::Exit) => self.exit(),
            Message::View(view::Message::CopyToClipboard(contents)) => {
                iced::clipboard::write(contents)
            }
            Message::Empty => Command::none(),
            // All the view messages are forwarded to the screen.
            _ => self.windows.screen.update(message),
        })
    }

    pub fn view(&self) -> Element<Message> {
        view::app_layout(&self.windows.sidebar, self.windows.screen.view()).map(Message::View)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        self.windows.screen.subscription()
    }

    pub fn exit(&mut self) -> Command<Message> {
        // Save the profile to disk.
        let result = self.storage.profile.save();
        match result {
            Ok(_) => tracing::info!("Saved profile to disk"),
            Err(e) => tracing::error!("Failed to save profile to disk: {:?}", e),
        }

        // Call exit on the opened window.
        let mut commands = Vec::new();
        let cmd = self.windows.screen.exit();
        commands.push(cmd);

        // If the dev client is Some, call the anvil client using `anvil_dumpState`, and
        // set the profile's anvil snapshot to the result.
        if let Some(_) = self.client.anvil {
            let cmd = Command::perform(save_snapshot(self.client.clone()), |result| {
                Message::StorageMessage(StorageMessage::AnvilSnapshot(result))
            });
            commands.push(cmd);
        }

        Command::batch(commands)
    }

    #[allow(unused_assignments)]
    fn cache_update(&mut self, message: CacheMessage) -> Command<Message> {
        let mut cmd = Command::none();
        match message {
            // Cannot use tracing here.
            CacheMessage::AppEvent(log) => {
                // Define the maximum number of logs
                const MAX_LOGS: usize = 100;

                // Push the new log
                self.cache.app_events.push_back(log);

                // If the number of data_feed exceeds the maximum, remove the oldest one
                if self.cache.app_events.len() > MAX_LOGS {
                    self.cache.app_events.pop_front();
                }

                // todo: figure out how to best pipe updated app state to windows...
                // todo: update, old.
                cmd = Command::perform(async {}, |_| Message::View(view::Message::Empty));
            }
        }
        cmd
    }

    fn contacts_update(&mut self, message: AddressBookMessage) -> Command<Message> {
        let cmd = Command::none();
        let contacts = &mut self.storage.profile.contacts;
        match message {
            // todo: update these messages
            AddressBookMessage::Add(name, address, category) => {
                contacts.add(
                    address,
                    ContactValue {
                        label: name,
                        ..Default::default()
                    },
                    category,
                );
            }
            AddressBookMessage::Remove(name, category) => {
                let address = name.parse::<Address>().unwrap();
                contacts.remove(&address, category);
            }
            AddressBookMessage::Get(name, category) => {
                let address = name.parse::<Address>().unwrap();
                contacts.get(&address, category);
            }
            AddressBookMessage::List(category) => {
                contacts.list(category);
            }
            AddressBookMessage::Clear(category) => {
                contacts.clear(category);
            }
        }
        cmd
    }

    fn rpcs_update(&mut self, message: RPCStorageMessage) -> Command<Message> {
        let profile = &mut self.storage.profile;
        match message {
            RPCStorageMessage::Add(chain) => {
                profile.rpcs.add(chain);
            }
            RPCStorageMessage::Remove(name) => {
                tracing::debug!("Removing RPC from storage: {}", name);
                profile.rpcs.remove(&name);
            }
            RPCStorageMessage::Get(name) => {
                profile.rpcs.get(&name);
            }
            RPCStorageMessage::List => {
                profile.rpcs.list();
            }
            RPCStorageMessage::Clear => {
                profile.rpcs.clear();
            }
        }

        // Make sure to save the new storage.
        let result = profile.save();
        match result {
            Ok(_) => tracing::info!("Saved profile to disk"),
            Err(e) => tracing::error!("Failed to save profile to disk: {:?}", e),
        }

        // todo: this can probably be removed.
        // we can just update the storage in the rpc settings manually.
        let rpcs = profile.rpcs.clone();
        tracing::info!("Syncing RPCs from app: {:?}", rpcs);
        Command::perform(async {}, move |_| {
            view::Message::Settings(settings::Message::Rpc(settings::rpc::Message::Sync(rpcs)))
        })
        .map(|x| x.into())
    }

    #[allow(unused_assignments)]
    fn storage_update(&mut self, message: StorageMessage) -> Command<Message> {
        let mut cmd = Command::none();
        match message {
            StorageMessage::AddressBook(msg) => {
                cmd = self.contacts_update(msg);
            }
            StorageMessage::RPCStorage(msg) => {
                cmd = self.rpcs_update(msg);
            }
            StorageMessage::AnvilSnapshot(snapshot) => {
                tracing::debug!("Saving anvil snapshot to profile");
                match snapshot {
                    Ok(snapshot) => {
                        self.storage.profile.anvil_snapshot = Some(snapshot);
                        tracing::debug!("Saved anvil snapshot to profile");
                    }
                    Err(e) => tracing::error!("Failed to save anvil snapshot: {:?}", e),
                }

                // Exits the application after saving the anvil snapshot.
                return Command::perform(async {}, |_| Message::Exit);
            }
        }
        cmd
    }

    // Forwards window messages to the screen.
    #[allow(unused_assignments)]
    fn windows_update(&mut self, message: WindowsMessage) -> Command<Message> {
        let mut cmd = Command::none();
        match message {
            WindowsMessage::Switch(route) => {
                cmd = self.switch_window(&route);
            }
            _ => cmd = self.windows.screen.update(Message::WindowsMessage(message)),
        }
        cmd
    }

    #[allow(unreachable_patterns)]
    fn switch_window(&mut self, navigate_to: &view::sidebar::Route) -> Command<Message> {
        let mut cmds = Vec::new();

        let exit_cmd = self.windows.screen.exit();
        cmds.push(exit_cmd);

        self.windows.screen = match navigate_to {
            view::sidebar::Route::Page(page) => {
                // Updates the current page in the sidebar.
                cmds.push(
                    self.windows
                        .sidebar
                        .update(view::sidebar::Route::Page(*page))
                        .map(|x| x.into()),
                );

                match page {
                    view::sidebar::Page::Empty => EmptyScreen::new().into(),
                    view::sidebar::Page::Portfolio => {
                        PortfolioRoot::new(Some(self.client.clone()), self.storage.profile.clone())
                            .into()
                    }
                    view::sidebar::Page::Settings => {
                        SettingsScreen::new(self.storage.clone()).into()
                    }
                    view::sidebar::Page::Exit => ExitScreen::new(true).into(),
                }
            }
            _ => EmptyScreen::new().into(),
        };

        let load_cmd = self.windows.screen.load();
        cmds.push(load_cmd);

        Command::batch(cmds)
    }
}

async fn save_snapshot(
    client: Arc<ExcaliburMiddleware<Ws, LocalWallet>>,
) -> anyhow::Result<String> {
    client.snapshot().await
}

#[cfg(test)]
mod tests {

    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    use super::*;

    fn cache_update_bench(c: &mut Criterion) {
        // let storage = Storage::default();
        // let chains = Chains::default();
        // let mut app = App::new(storage, chains, Streams::default(), None).0;
        // c.bench_function("cache_update", |b| {
        // b.iter(|| {
        // app.cache_update(CacheMessage::AppEvent(AppEventLog::new(
        // "test".to_string(),
        // "test".to_string(),
        // )))
        // })
        // });
    }
}
