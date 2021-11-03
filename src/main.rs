mod ftw_build_type;
mod ftw_command;
mod ftw_configuration;
mod ftw_error;
mod ftw_machine_type;
mod ftw_node_type;
mod ftw_success;
mod ftw_target;
mod ftw_template;
mod run_command;
mod test_util;
mod traits;
mod type_alias;
mod util;

use crate::ftw_command::FtwCommand;
use crate::traits::{Processor, ToMessage};
use clap::{crate_authors, crate_name, crate_version, App, ArgMatches, Arg};
use std::env;

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), ()> {
    let matches = get_clap_app()
        .get_matches();
    let command = parse_matches(&matches);
    command
        .process()
        .map(|ftw_success| println!("{}", ftw_success.to_message()))
        .map_err(|ftw_error| eprintln!("{}", ftw_error.to_message()))
}

fn get_clap_app() -> App<'static> {
    let version = crate_version!();
    let author = crate_authors!("\n");
    App::new(crate_name!())
        .version(version)
        .author(author)
        .about("manage your godot-rust project")
        .subcommand(App::new("new")
            .about("create a new godot-rust project directory")
            .arg(Arg::new("project_name").required(true).about("set the name of your project"))
            .arg(Arg::new("template").required(false).about("set the template to be used in your project"))
        )
        .subcommand(App::new("class")
            .about("create a new class to be used by a node")
            .arg(Arg::new("class_name").required(true).about("the name of this class"))
            .arg(Arg::new("node_type").required(false).about("the type of the node that this class inherits from"))
        )
        .subcommand(App::new("singleton")
            .about("create a singleton (autoloaded) class")
            .arg(Arg::new("class_name").required(true).about("the name of this class"))
        )
        .subcommand(App::new("run")
            .about("run a debug version of the game")
            .arg(Arg::new("machine_type").required(false).about("either desktop or server"))
        )
        .subcommand(App::new("build")
            .about("build the library for a particular platform")
            .arg(Arg::new("target").required(false).about("target platform to build"))
            .arg(Arg::new("build_type").required(false).about("either a debug or release"))
        )
        .subcommand(App::new("export")
            .about("export the game for a particular platform")
            .arg(Arg::new("target").required(false).about("target platform to build"))
            .arg(Arg::new("build_type").required(false).about("either a debug or release"))
        )
}

fn parse_matches(matches: &ArgMatches) -> FtwCommand {
    match matches.subcommand() {
        Some(("new", args)) => {
            let project_name = args
                .value_of("project_name")
                .unwrap_or("my-awesome-game")
                .to_string();
            let template = args
                .value_of("template")
                .unwrap_or("default")
                .parse()
                .unwrap_or_default();
            FtwCommand::New {
                project_name,
                template,
            }
        }
        Some(("class", args)) => {
            let class_name = args.value_of("class_name").unwrap_or("MyClass").to_string();
            let node_type = args
                .value_of("node_type")
                .unwrap_or("Node")
                .parse()
                .unwrap_or_default();
            FtwCommand::Class {
                class_name,
                node_type,
            }
        }
        Some(("singleton", args)) => {
            let class_name = args
                .value_of("class_name")
                .unwrap_or("MySingletonClass")
                .to_string();
            FtwCommand::Singleton { class_name }
        }
        Some(("run", args)) => {
            let machine_type = args
                .value_of("machine_type")
                .unwrap_or("desktop")
                .parse()
                .unwrap_or_default();
            FtwCommand::Run { machine_type }
        }
        Some(("build", args)) => {
            let current_platform = util::get_current_platform();
            let target = args
                .value_of("target")
                .unwrap_or(&current_platform)
                .parse()
                .unwrap_or_default();
            let build_type = args
                .value_of("build_type")
                .unwrap_or("debug")
                .parse()
                .unwrap_or_default();
            FtwCommand::Build { target, build_type }
        }
        Some(("export", args)) => {
            let current_platform = util::get_current_platform();
            let target = args
                .value_of("target")
                .unwrap_or(&current_platform)
                .parse()
                .unwrap_or_default();
            let build_type = args
                .value_of("build_type")
                .unwrap_or("debug")
                .parse()
                .unwrap_or_default();
            FtwCommand::Export { target, build_type }
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod main_tests {
    use super::*;
    use crate::ftw_build_type::FtwBuildType;
    use crate::ftw_command::FtwCommand;
    use crate::ftw_machine_type::FtwMachineType;
    use crate::ftw_node_type::FtwNodeType;
    use crate::ftw_target::FtwTarget;
    use crate::ftw_template::FtwTemplate;
    use crate::util;

    #[test]
    fn test_parse_matches_new() {
        let app = get_clap_app();
        let project_name = "my-awesome-game";
        let arg_vec = vec![crate_name!(), "new", project_name, "default"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::New {
            project_name: project_name.to_string(),
            template: FtwTemplate::default(),
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_new_no_template() {
        let app = get_clap_app();
        let project_name = "my-awesome-game";
        let arg_vec = vec![crate_name!(), "new", project_name];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::New {
            project_name: project_name.to_string(),
            template: FtwTemplate::default(),
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_new_custom_template() {
        let app = get_clap_app();
        let project_name = "my-awesome-game";
        let git_url = "/path/to/custom/template";
        let arg_vec = vec![crate_name!(), "new", project_name, git_url];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::New {
            project_name: project_name.to_string(),
            template: FtwTemplate::Custom {
                git_url: git_url.to_string(),
            },
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_class() {
        let app = get_clap_app();
        let class_name = "IronMan";
        let arg_vec = vec![crate_name!(), "class", class_name, "Area2D"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Class {
            class_name: class_name.to_string(),
            node_type: FtwNodeType::Area2D,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_class_no_node_type() {
        let app = get_clap_app();
        let class_name = "IronMan";
        let arg_vec = vec![crate_name!(), "class", class_name];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Class {
            class_name: class_name.to_string(),
            node_type: FtwNodeType::Node,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_singleton() {
        let app = get_clap_app();
        let class_name = "Network";
        let arg_vec = vec![crate_name!(), "singleton", class_name];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Singleton {
            class_name: class_name.to_string(),
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_run_desktop() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "run", "desktop"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Run {
            machine_type: FtwMachineType::Desktop,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_run_server() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "run", "server"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Run {
            machine_type: FtwMachineType::Server,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_run_no_machine_type() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "run"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Run {
            machine_type: FtwMachineType::Desktop,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_build() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "build", "linux-x86_64", "debug"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Build {
            target: FtwTarget::LinuxX86_64,
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_build_no_build_type() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "build", "linux-x86_64"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Build {
            target: FtwTarget::LinuxX86_64,
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_build_no_target_and_no_build_type() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "build"];
        let target = util::get_current_platform().parse().unwrap();
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Build {
            target,
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_export() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "export", "linux-x86_64", "debug"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Export {
            target: FtwTarget::LinuxX86_64,
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_export_no_build_type() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "export", "linux-x86_64"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Export {
            target: FtwTarget::LinuxX86_64,
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_export_no_target_and_no_build_type() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "export"];
        let target = util::get_current_platform().parse().unwrap();
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Export {
            target,
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }
}
