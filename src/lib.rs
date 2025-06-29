//! Common definitions for the client and server

use clap::{CommandFactory, FromArgMatches};
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum CommunicationProtocol {
    Http,
    Grpc,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLineArgs {
    /// The protocol to use
    #[arg(short, long, value_enum, default_value_t = CommunicationProtocol::Http)]
    pub comm_protocol: CommunicationProtocol,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInformation {
    pub ram_usage: u64,
}

/// Helper to parse with custom about text
pub fn create_command_line_arg_parser(about_text: String) -> CommandLineArgs {
    let mut cmd = CommandLineArgs::command();
    cmd = cmd.about(about_text);
    CommandLineArgs::from_arg_matches(&cmd.get_matches()).unwrap()
}
