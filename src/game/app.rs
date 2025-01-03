// This has basically been ripped from the lightyear examples
// There's going to need to be a lot of work done to make it fit our specifics
// TODO: Massive refactor of this whole thing

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use bevy::asset::ron;
use bevy::log::{ Level, LogPlugin };
use bevy::prelude::*;

use bevy::state::app::StatesPlugin;
use bevy::DefaultPlugins;
use clap::{ Parser, ValueEnum };
use lightyear::prelude::client::ClientConfig;
use lightyear::prelude::*;
use lightyear::prelude::{ client, server };
use lightyear::server::config::ServerConfig;
use lightyear::shared::log::add_log_layer;
use lightyear::transport::LOCAL_SOCKET;
use serde::{ Deserialize, Serialize };

use crate::utils::settings::*;
use crate::network::shared::*;

#[cfg(feature = "gui")]
use crate::render::ui::UiRenderPlugin;
#[cfg(feature = "gui")]
use bevy::window::PresentMode;

#[derive(Parser, PartialEq, Debug)]
pub struct Cli {
    #[cfg(feature = "client")]
    #[arg(short, long)]
    client_id: Option<u64>,

    #[cfg(all(feature = "client", feature = "server"))]
    #[arg(short, long, default_value = "host-server", value_enum)]
    pub mode: ServerMode,
}

// Parses cli args so that they can be used to configure compilation etc.
pub fn cli() -> Cli {
    info!("Parsing command-line arguments.");
    let parsed_cli = Cli::parse();
    info!("Parsed CLI arguments: {:?}", parsed_cli);
    parsed_cli
}

#[derive(ValueEnum, Clone, Default, Debug, PartialEq)]
pub enum ServerMode {
    #[default]
    HostServer,
    Separate,
}

#[derive(Debug)]
struct SendApp(App);

unsafe impl Send for SendApp {}
impl SendApp {
    fn run(&mut self) {
        info!("SendApp: Starting to run the encapsulated App.");
        self.0.run();
    }
}

pub enum Apps {
    Client {
        app: App,
        config: ClientConfig,
    },
    Server {
        app: App,
        config: ServerConfig,
    },
    ClientAndServer {
        client_app: App,
        client_config: ClientConfig,
        server_app: App,
        server_config: ServerConfig,
    },
    HostServer {
        app: App,
        client_config: ClientConfig,
        server_config: ServerConfig,
    },
}

impl Apps {

    // Build the apps with the given settings and CLI optionsa.
    pub fn new(settings: Settings, cli: Cli, name: String) -> Option<Self> {
        info!(
            "Initializing Apps::new with settings: {:?}, cli: {:?}, name: {}",
            settings,
            cli,
            name
        );
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "client", feature = "server"))] {
                info!("Both client and server features are enabled.");
                match cli.mode {
                    ServerMode::HostServer => {
                        info!("Mode selected: HostServer");
                        let client_net_config = client::NetConfig::Local {
                            id: cli.client_id.unwrap_or(settings.client.client_id),
                        };
                        info!("Client network configuration fetched");
                        let (app, client_config, server_config) = combined_app(
                            settings,
                            vec![],
                            client_net_config
                        );
                        info!("Combined App created successfully.");
                        Some(Apps::HostServer {
                            app,
                            client_config,
                            server_config,
                        })
                    }
                    ServerMode::Separate => {
                        info!("Mode selected: Separate");
                        // we will communicate between the client and server apps via channels
                        let (from_server_send, from_server_recv) = crossbeam_channel::unbounded();
                        let (to_server_send, to_server_recv) = crossbeam_channel::unbounded();
                        info!("Channels for client-server communication established.");
                        let transport_config = client::ClientTransport::LocalChannel {
                            recv: from_server_recv,
                            send: to_server_send,
                        };
                        info!("Client transport configuration: {:?}", transport_config);

                        // create client app
                        let net_config = build_client_netcode_config(
                            cli.client_id.unwrap_or(settings.client.client_id),
                            // when communicating via channels, we need to use the address `LOCAL_SOCKET` for the server
                            LOCAL_SOCKET,
                            settings.client.conditioner.as_ref(),
                            &settings.shared,
                            transport_config
                        );
                        info!("Client network configuration built");
                        let (client_app, client_config) = client_app(settings.clone(), net_config);
                        info!("Client App created successfully.");
                        // create server app, which will be headless when we have client app in same process
                        let extra_transport_configs = vec![server::ServerTransport::Channels {
                            // even if we communicate via channels, we need to provide a socket address for the client
                            channels: vec![(LOCAL_SOCKET, to_server_recv, from_server_send)],
                        }];
                        info!("Server transport configurations: {:?}", extra_transport_configs);
                        let (server_app, server_config) = server_app(
                            settings,
                            extra_transport_configs
                        );
                        info!("Server App created successfully.");
                        Some(Apps::ClientAndServer {
                            client_app,
                            client_config,
                            server_app,
                            server_config,
                        })
                    }
                }
            } else if #[cfg(feature = "server")] {
                info!("Only server feature is enabled.");
                #[allow(unused_mut)]
                let (mut app, config) = server_app(settings, vec![]);
                info!("Server App created successfully.");
                Some(Apps::Server { app, config })
            } else if #[cfg(feature = "server")] {
                info!("Only client feature is enabled.");
                let server_addr = SocketAddr::new(
                    settings.client.server_addr.into(),
                    settings.client.server_port
                );
                info!("Server address for client: {}", server_addr);
                let client_id = cli.client_id.unwrap_or(settings.client.client_id);
                info!("Client ID: {}", client_id);
                let net_config = get_client_net_config(&settings, client_id);
                info!("Client network configuration recieved");
                let (app, config) = client_app(settings, net_config);
                info!("Client App created successfully.");
                Some(Apps::Client { app, config })
            } else {None}
        }
    }

    /// Set the `server_replication_send_interval` on client and server apps.
    /// Use to overwrite the default [`SharedConfig`] value in the settings file.
    pub fn with_server_replication_send_interval(mut self, replication_interval: Duration) -> Self {
        info!("Setting server replication send interval to {:?}", replication_interval);
        self.update_lightyear_client_config(|cc: &mut ClientConfig| {
            cc.shared.server_replication_send_interval = replication_interval;
            info!(
                "Updated ClientConfig.server_replication_send_interval to {:?}",
                replication_interval
            );
        });
        self.update_lightyear_server_config(|sc: &mut ServerConfig| {
            // the server replication currently needs to be overwritten in both places...
            sc.shared.server_replication_send_interval = replication_interval;
            sc.replication.send_interval = replication_interval;
            info!(
                "Updated ServerConfig.server_replication_send_interval and replication.send_interval to {:?}",
                replication_interval
            );
        });
        self
    }

    /// Add the lightyear [`ClientPlugins`] and [`ServerPlugins`] plugin groups to the app.
    /// This can be called after any modifications to the [`ClientConfig`] and [`ServerConfig`]
    /// have been applied.
    pub fn add_lightyear_plugins(&mut self) -> &mut Self {
        info!("Adding Lightyear plugins to the App(s).");
        match self {
            Apps::Client { app, config } => {
                info!("Adding ClientPlugins to Client App.");
                app.add_plugins(client::ClientPlugins {
                    config: config.clone(),
                });
            }
            Apps::Server { app, config } => {
                info!("Adding ServerPlugins to Server App.");
                app.add_plugins(server::ServerPlugins {
                    config: config.clone(),
                });
            }
            Apps::ClientAndServer { client_app, server_app, client_config, server_config } => {
                info!("Adding ClientPlugins to Client App.");
                client_app.add_plugins(client::ClientPlugins {
                    config: client_config.clone(),
                });
                info!("Adding ServerPlugins to Server App.");
                server_app.add_plugins(server::ServerPlugins {
                    config: server_config.clone(),
                });
            }
            Apps::HostServer { app, client_config, server_config } => {
                // TODO: currently we need ServerPlugins to run first, because it adds the
                //  SharedPlugins. not ideal
                info!("Adding ClientPlugins to HostServer App.");
                app.add_plugins(client::ClientPlugins {
                    config: client_config.clone(),
                });
                info!("Adding ServerPlugins to HostServer App.");
                app.add_plugins(server::ServerPlugins {
                    config: server_config.clone(),
                });
            }
        }

        self
    }

    /// Adds plugin to the client app
    pub fn add_user_client_plugin(&mut self, client_plugin: impl Plugin) -> &mut Self {
        match self {
            Apps::Client { app, .. } => {
                app.add_plugins(client_plugin);
                info!("User-defined Client plugin added to Client App.");
            }
            Apps::ClientAndServer { client_app, .. } => {
                client_app.add_plugins(client_plugin);
                info!("User-defined Client plugin added to Client App within ClientAndServer.");
            }
            Apps::HostServer { app, .. } => {
                app.add_plugins(client_plugin);
                info!("User-defined Client plugin added to HostServer App.");
            }
            Apps::Server { .. } => {
                info!("No Client App present. Plugin not added.");
            }
        }
        self
    }

    /// Adds plugin to the server app
    pub fn add_user_server_plugin(&mut self, server_plugin: impl Plugin) -> &mut Self {
        match self {
            Apps::Client { .. } => {
                info!("No Server App present. Plugin not added.");
            }
            Apps::ClientAndServer { server_app, .. } => {
                server_app.add_plugins(server_plugin);
                info!("User-defined Server plugin added to Server App within ClientAndServer.");
            }
            Apps::HostServer { app, .. } => {
                app.add_plugins(server_plugin);
                info!("User-defined Server plugin added to HostServer App.");
            }
            Apps::Server { app, .. } => {
                app.add_plugins(server_plugin);
                info!("User-defined Server plugin added to Server App.");
            }
        }
        self
    }

    /// Adds plugin to both the server and client apps, if present
    pub fn add_user_shared_plugin(&mut self, shared_plugin: impl Plugin + Clone) -> &mut Self {
        match self {
            Apps::Client { app, config } => {
                app.add_plugins(shared_plugin);
                info!("User-defined Shared plugin added to Client App.");
            }
            Apps::ClientAndServer { server_app, client_app, .. } => {
                server_app.add_plugins(shared_plugin.clone());
                client_app.add_plugins(shared_plugin);
                info!(
                    "User-defined Shared plugin added to both Client and Server Apps within ClientAndServer."
                );
            }
            Apps::HostServer { app, .. } => {
                app.add_plugins(shared_plugin);
                info!("User-defined Shared plugin added to HostServer App.");
            }
            Apps::Server { app, .. } => {
                app.add_plugins(shared_plugin);
                info!("User-defined Shared plugin added to Server App.");
            }
        }
        self
    }

    /// Adds to the client app, and the server app if in standalone server mode with the cargo "gui" feature.
    /// Won't add renderer to server app if a client app also present.
    pub fn add_user_renderer_plugin(&mut self, renderer_plugin: impl Plugin) -> &mut Self {
        match self {
            Apps::Client { app, config } => {
                app.add_plugins(renderer_plugin);
                info!("User-defined Renderer plugin added to Client App.");
            }
            Apps::ClientAndServer { server_app, client_app, .. } => {
                client_app.add_plugins(renderer_plugin);
                info!("User-defined Renderer plugin added to Client App within ClientAndServer.");
            }
            Apps::HostServer { app, .. } => {
                app.add_plugins(renderer_plugin);
                info!("User-defined Renderer plugin added to HostServer App.");
            }
            Apps::Server { app, .. } => {
                app.add_plugins(renderer_plugin);
                info!("User-defined Renderer plugin added to Server App.");
            }
        }
        self
    }

    /// Add the client, server, and shared user-provided plugins to the app
    pub fn add_user_plugins(
        &mut self,
        client_plugin: impl Plugin,
        server_plugin: impl Plugin,
        shared_plugin: impl Plugin + Clone,
        renderer_plugin: impl Plugin
    ) -> &mut Self {
        info!("Adding user-provided plugins to App(s).");
        self.add_user_shared_plugin(shared_plugin);
        #[cfg(feature = "client")]
        self.add_user_client_plugin(client_plugin);
        #[cfg(feature = "server")]
        self.add_user_server_plugin(server_plugin);
        #[cfg(feature = "gui")]
        self.add_user_renderer_plugin(renderer_plugin);
        info!("User-provided plugins added successfully.");
        self
    }

    /// Apply a function to update the [`ClientConfig`]
    pub fn update_lightyear_client_config(
        &mut self,
        f: impl FnOnce(&mut ClientConfig)
    ) -> &mut Self {
        info!("Updating ClientConfig with provided function.");
        match self {
            Apps::Client { config, .. } => {
                f(config);
                info!("ClientConfig updated");
            }
            Apps::Server { config, .. } => {
                info!("No ClientConfig present to update.");
            }
            Apps::ClientAndServer { client_config, .. } => {
                f(client_config);
                info!("ClientConfig updated within ClientAndServer");
            }
            Apps::HostServer { client_config, .. } => {
                f(client_config);
                info!("ClientConfig updated within HostServer");
            }
        }
        self
    }

    /// Apply a function to update the [`ServerConfig`]
    pub fn update_lightyear_server_config(
        &mut self,
        f: impl FnOnce(&mut ServerConfig)
    ) -> &mut Self {
        info!("Updating ServerConfig with provided function.");
        match self {
            Apps::Client { config, .. } => {
                info!("No ServerConfig present to update.");
            }
            Apps::Server { config, .. } => {
                f(config);
                info!("ServerConfig updated: {:?}", config);
            }
            Apps::ClientAndServer { server_config, .. } => {
                f(server_config);
                info!("ServerConfig updated within ClientAndServer: {:?}", server_config);
            }
            Apps::HostServer { server_config, .. } => {
                f(server_config);
                info!("ServerConfig updated within HostServer: {:?}", server_config);
            }
        }
        self
    }

    /// Start running the apps.
    pub fn run(self) {
        info!("Running the App(s).");
        match self {
            Apps::Client { mut app, .. } => {
                info!("Running Client App.");
                app.run();
            }
            Apps::Server { mut app, .. } => {
                info!("Running Server App.");
                app.run();
            }
            Apps::ClientAndServer { mut client_app, server_app, .. } => {
                info!("Running ClientAndServer App.");
                info!("Before spawning thread: {:?}, {:?}", server_app, client_app);
                let mut send_app = SendApp(server_app);
                info!("After encapsulating Server App into SendApp: {:?}", send_app);
                std::thread::spawn(move || {
                    info!("Spawned thread for Server App.");
                    info!("Running Server App.");
                    send_app.run();
                });
                info!("Running Client App.");
                client_app.run();
            }
            Apps::HostServer { mut app, .. } => {
                info!("Running HostServer App.");
                app.run();
            }
        }
    }
}

#[cfg(feature = "gui")]
fn window_plugin() -> WindowPlugin {
    info!("Configuring WindowPlugin for GUI.");
    WindowPlugin {
        primary_window: Some(Window {
            title: format!("Lightyear Example: {}", env!("CARGO_PKG_NAME")),
            resolution: (1024.0, 768.0).into(),
            present_mode: PresentMode::AutoVsync,
            prevent_default_event_handling: true,
            ..Default::default()
        }),
        ..default()
    }
}

fn log_plugin() -> LogPlugin {
    info!("Setting up LogPlugin with INFO level and custom filters.");
    LogPlugin {
        level: Level::INFO,
        filter: "wgpu=error,bevy_render=info,bevy_ecs=warn".to_string(),
        ..default()
    }
}

#[cfg(feature = "gui")]
fn new_gui_app(add_inspector: bool) -> App {
    info!("Creating new GUI App. Add inspector: {}", add_inspector);
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins.build()
            .set(AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            })
            .set(log_plugin())
            .set(window_plugin())
    );
    info!("Default plugins added to GUI App.");
    if add_inspector {
        info!("Adding WorldInspectorPlugin to GUI App.");
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
    }
    app
}

fn new_headless_app() -> App {
    info!("Creating new Headless App.");
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, log_plugin(), StatesPlugin, HierarchyPlugin));
    info!("Minimal plugins added to Headless App.");
    app
}

/// Build the client app with the `ClientPlugins` added.
/// Takes in a `net_config` parameter so that we configure the network transport.
#[cfg(feature = "client")]
fn client_app(settings: Settings, net_config: client::NetConfig) -> (App, ClientConfig) {
    info!("Building Client App with settings: {:?} and netconfig", settings);
    let app = new_gui_app(settings.client.inspector);
    info!("GUI App for Client created.");

    let client_config = ClientConfig {
        shared: shared_config(Mode::Separate),
        net: net_config,
        replication: ReplicationConfig {
            send_interval: SERVER_REPLICATION_INTERVAL,
            ..default()
        },
        ..default()
    };
    info!("ClientConfig initialized");
    (app, client_config)
}

/// Build the server app with the `ServerPlugins` added.
#[cfg(feature = "server")]
fn server_app(
    settings: Settings,
    extra_transport_configs: Vec<server::ServerTransport>
) -> (App, ServerConfig) {
    use cfg_if::cfg_if;
    info!(
        "Building Server App with settings: {:?}, extra_transport_configs: {:?}",
        settings,
        extra_transport_configs
    );
    cfg_if::cfg_if! {
        // If there's a client app, the server needs to be headless.
        // Winit doesn't support two event loops in the same thread.
        if #[cfg(feature = "client")] {
            info!("Client feature is enabled. Creating Headless App for Server.");
            let app = new_headless_app();
        } else if #[cfg(feature = "gui")] {
            info!("GUI feature is enabled. Creating GUI App for Server.");
            let app = new_gui_app(settings.server.inspector);
        } else {
            info!("Creating Headless App for Server.");
            let app = new_headless_app();
        }
    }
    // configure the network configuration
    let mut net_configs = get_server_net_configs(&settings);
    info!("Initial Server network configurations");
    let extra_net_configs = extra_transport_configs
        .into_iter()
        .map(|c| {
            build_server_netcode_config(settings.server.conditioner.as_ref(), &settings.shared, c)
        });
    net_configs.extend(extra_net_configs);
    info!("Extended Server network configurations with extra transports");
    let server_config = ServerConfig {
        shared: shared_config(Mode::Separate),
        net: net_configs,
        replication: ReplicationConfig {
            send_interval: SERVER_REPLICATION_INTERVAL,
            ..default()
        },
        ..default()
    };
    info!("ServerConfig initialized: {:?}", server_config);
    (app, server_config)
}

/// An `App` that contains both the client and server plugins
#[cfg(all(feature = "client", feature = "server"))]
fn combined_app(
    settings: Settings,
    extra_transport_configs: Vec<server::ServerTransport>,
    client_net_config: client::NetConfig
) -> (App, ClientConfig, ServerConfig) {
    info!(
        "Building Combined App with settings: {:?}, extra_transport_configs: {:?}, and client netconfig",
        settings,
        extra_transport_configs
    );
    let app = new_gui_app(settings.client.inspector || settings.server.inspector);
    info!("GUI App for Combined App created.");
    // server config
    let mut net_configs = get_server_net_configs(&settings);
    info!("Initial Combined App server network configurations recieved");
    let extra_net_configs = extra_transport_configs
        .into_iter()
        .map(|c| {
            build_server_netcode_config(settings.server.conditioner.as_ref(), &settings.shared, c)
        });
    net_configs.extend(extra_net_configs);
    info!("Extended Combined App server network configurations with extra transports");
    let server_config = ServerConfig {
        shared: shared_config(Mode::HostServer),
        net: net_configs,
        replication: ReplicationConfig {
            send_interval: SERVER_REPLICATION_INTERVAL,
            ..default()
        },
        ..default()
    };
    info!("Combined App ServerConfig initialized: {:?}", server_config);

    // client config
    let client_config = ClientConfig {
        shared: shared_config(Mode::HostServer),
        net: client_net_config,
        ..default()
    };
    info!("Combined App ClientConfig initialized");
    (app, client_config, server_config)
}
