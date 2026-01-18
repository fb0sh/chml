use crate::{command::Cli, config::AppHome, util::print};
use chml_api::ChmlApi;
use clap::Parser;

mod command;
mod config;
mod procedure;
mod util;

const CLI_NAME: &str = "chml";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_home = AppHome::new(CLI_NAME)?;
    app_home.ensure()?;

    let cli = Cli::parse();
    procedure::prepare_frpc(&app_home, cli.quiet).await?;

    match ChmlApi::from_env() {
        Ok(chml) => {
            if !cli.quiet {
                let user = chml.user_info().await?.into_result()?;
                print::print_user_info(&user);
                println!();
            }

            procedure::handle_command(cli, &app_home, &chml).await?;
        }
        Err(e) => {
            eprintln!("[-] {}", e.to_string());
            eprintln!("[*] You need set env var: [ CHML_API_BASE_URL, CHML_API_TOKEN ]");
        }
    }

    Ok(())
}

//
// ls 已有隧道和域名列表 nodes
//  connteced 在前面
// ls -t/--tunnels
// ls -d/--domains
// ls -n/--nodes
// 如果有VIP 则优先VIP

// add tunnel -t [name] [-lhost 127.0.0.1] -lport 4444 [-n node_name/default_closete] [-rport 88888/defualt random]
// add domain -r [recode.maindomain.dt] -dt [A] -ttl -rhost 1.1.1.1

// connect -t name
// connect -t name -d/--daemon

// rm --tunnel-id id
// rm -t name
// rm -d name

// get config -t tunnel_name

// shortcut
// tcp 4444
// http 8080 可建站 同时给出 http://ip:port/ 有domain or ip 都要给出

// 无需求改
