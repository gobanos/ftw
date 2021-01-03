mod ftw_command;
mod ftw_error;
mod ftw_template;
mod node_type;
mod process_command;
mod traits;
mod type_alias;

use crate::ftw_command::FtwCommand;
use crate::ftw_template::FtwTemplate;
use crate::node_type::NodeType;
use crate::traits::Processor;
use clap::{clap_app, crate_authors, crate_version};

fn main() {
    let version = crate_version!();
    let author = crate_authors!("\n");
    let matches = clap_app!(ftw =>
                            (version: version)
                            (author: author)
                            (about: "manage your godot-rust project")
                            (@subcommand new =>
                             (about: "create a new godot-rust project directory")
                             (@arg project_name: +required "set the name of your project")
                             (@arg template: !required "set the template to be used in your project"))
                            (@subcommand class =>
                             (about: "create a new class to be used by a node")
                             (@arg class_name: +required "set the name of your project")
                             (@arg node_type: !required "the type of the node that this class inherits from")))
    .get_matches();
    let command: FtwCommand = match matches.subcommand() {
        Some(("new", args)) => {
            let project_name = args
                .value_of("project_name")
                .unwrap_or_else(|| "my-awesome-game")
                .to_string();
            let template: FtwTemplate = args
                .value_of("template")
                .unwrap_or_else(|| "")
                .parse()
                .unwrap_or_else(|_| FtwTemplate::Default);
            FtwCommand::New {
                project_name: project_name,
                template: template,
            }
        }
        Some(("class", args)) => {
            let class_name = args
                .value_of("class_name")
                .unwrap_or_else(|| "MyClass")
                .to_string();
            let node_type: NodeType = args
                .value_of("node_type")
                .unwrap_or_else(|| "")
                .parse()
                .unwrap_or_else(|_| NodeType::Node);
            FtwCommand::Class {
                class_name: class_name,
                node_type: node_type,
            }
        }
        _ => unreachable!(),
    };
    if let Err(e) = command.process() {
        eprintln!("{}", e);
    }
}
