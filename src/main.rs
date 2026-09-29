use std::collections::HashMap;
use std::process::ExitCode;

use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{CommandFactory, Parser};

mod cli;
mod i18n;
mod process;
mod sockets;
mod table;

use cli::{format_target, parse_target, Cli, Commands};
use process::{kill_process, process_name};
use sockets::{all_sockets, socket_matches, socket_row};
use table::print_table;

fn run_find(target_str: &str) -> Result<(), String> {
    let (target, proto) = parse_target(target_str)?;
    let sockets = all_sockets()?;
    let matched: Vec<_> = sockets
        .iter()
        .filter(|s| socket_matches(target, proto, s))
        .collect();
    if matched.is_empty() {
        println!("{}", i18n::not_found(&format_target(&target, proto)));
        return Ok(());
    }
    let mut names = HashMap::new();
    let rows: Vec<Vec<String>> = matched.iter().map(|s| socket_row(s, &mut names)).collect();
    print_table(&rows);
    Ok(())
}

fn run_kill(target_str: &str) -> Result<(), String> {
    let (target, proto) = parse_target(target_str)?;
    let sockets = all_sockets()?;
    let matched: Vec<_> = sockets
        .iter()
        .filter(|s| socket_matches(target, proto, s))
        .collect();
    if matched.is_empty() {
        println!("{}", i18n::not_found(&format_target(&target, proto)));
        return Ok(());
    }
    let mut pids: Vec<u32> = matched
        .iter()
        .flat_map(|s| s.associated_pids.iter().copied())
        .collect();
    pids.sort();
    pids.dedup();
    if pids.is_empty() {
        return Err(i18n::cannot_determine_pid());
    }

    let mut failed = 0usize;
    for pid in pids {
        // 必须在 kill 之前取进程名，进程被杀后名字将不可查
        let name = process_name(pid);
        match kill_process(pid) {
            Ok(()) => println!("{}", i18n::killed(pid, &name)),
            Err(e) => {
                eprintln!("{}", i18n::kill_failed(pid, &name, &e));
                failed += 1;
            }
        }
    }
    if failed > 0 {
        Err(i18n::kill_partial(failed))
    } else {
        Ok(())
    }
}

fn run_list() -> Result<(), String> {
    let sockets = all_sockets()?;
    let mut names = HashMap::new();
    let mut rows: Vec<Vec<String>> = sockets.iter().map(|s| socket_row(s, &mut names)).collect();
    rows.sort_by(|a, b| a[2].cmp(&b[2]).then_with(|| a[3].cmp(&b[3])));
    print_table(&rows);
    Ok(())
}

fn main() -> ExitCode {
    // 管道输出被提前关闭（如 list | head）时按常规方式终止，而不是 panic
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            let kind = e.kind();
            let info = match kind {
                ErrorKind::UnknownArgument | ErrorKind::InvalidSubcommand => {
                    let mut name = None;
                    let mut suggestion = None;
                    for (ck, cv) in e.context() {
                        match (ck, cv) {
                            (ContextKind::InvalidArg, ContextValue::String(s))
                            | (ContextKind::InvalidSubcommand, ContextValue::String(s)) => {
                                name = Some(s.clone())
                            }
                            (ContextKind::SuggestedArg, ContextValue::String(s)) => {
                                suggestion = Some(s.clone())
                            }
                            (ContextKind::SuggestedSubcommand, ContextValue::Strings(v)) => {
                                suggestion = v.first().cloned()
                            }
                            _ => {}
                        }
                    }
                    name.map(|n| (n, suggestion))
                }
                _ => None,
            };
            match info {
                Some((name, suggestion)) => {
                    let msg = if kind == ErrorKind::InvalidSubcommand {
                        i18n::unexpected_subcommand(&name, suggestion.as_deref())
                    } else {
                        i18n::unexpected_arg(&name, suggestion.as_deref())
                    };
                    eprintln!("{msg}");
                    return ExitCode::FAILURE;
                }
                None => e.exit(),
            }
        }
    };
    let result = match cli.sub_command {
        Some(Commands::Kill { target }) => run_kill(&target),
        Some(Commands::Find { target }) => run_find(&target),
        Some(Commands::List) => run_list(),
        None => {
            let _ = Cli::command().print_help();
            return ExitCode::SUCCESS;
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
