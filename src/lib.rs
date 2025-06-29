//! Common definitions for the client and server

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum CommunicationProtocol {
    Http,
    Grpc,
}

/// Simple program to greet a person
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
