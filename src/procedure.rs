use std::{cmp, ops::Deref};

use crate::{
    command::{Add, Cli, Commands},
    config::{self, AppHome},
    util::{self, print},
};
use anyhow::anyhow;
use chml_api::{
    ChmlApi, domain::function::CreateDomainParams, schema, tunnel::function::CreateTunnelParams,
};
use tokio::signal;
use tokio::{
    fs,
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

pub async fn prepare_frpc(app_home: &AppHome, is_quiet: bool) -> anyhow::Result<()> {
    // 简化静默模式打印
    macro_rules! log {
        ($($arg:tt)*) => {
            if !is_quiet {
                println!($($arg)*);
            }
        };

    }
    log!("[*] Chmlfrp Panel: https://panel.chmlfrp.net/");
    // 确保 bin 目录存在
    let bin_dir = app_home.join_dir("bin")?;
    tokio::fs::create_dir_all(&bin_dir).await?;
    let bin_path = bin_dir.join(config::bin_name());

    if !bin_path.exists() {
        log!(
            "[-] {} not found, downloading binary...",
            bin_path.display()
        );
        util::download_fpc_client(&bin_path).await?;
        log!("[+] frpc has been downloaded at {}", bin_dir.display());
    } else {
        log!("[*] frpc binary is ready at {}", bin_path.display());
    }

    // 确保 conf 目录存在
    let conf_dir = app_home.join_dir("conf")?;
    tokio::fs::create_dir_all(&conf_dir).await?;
    log!("[*] frpc config directory is in: {}", conf_dir.display());

    Ok(())
}

pub async fn handle_command(cli: Cli, app_home: &AppHome, chml: &ChmlApi) -> anyhow::Result<()> {
    match cli.command {
        // -t -d -n -tdn
        Commands::Ls {
            tunnels,
            domains,
            nodes,
            configs,
        } => {
            let mut tunnels = tunnels;
            let mut domains = domains;
            let mut configs = configs;
            let nodes = nodes;

            // 如果用户没选任何选项，默认展示 tunnels + domains
            if !tunnels && !domains && !nodes && !configs {
                tunnels = true;
                domains = true;
                configs = true;
            }

            if tunnels {
                let tunnels = chml.tunnel().await?.into_result()?;
                print::print_user_tunnels(&tunnels);
                println!();
            }

            if domains {
                let domains = chml.get_user_free_domains().await?.into_result()?;
                print::print_user_domains(&domains);
                println!();
            }

            if nodes {
                let nodes = chml.node().await?.into_result()?;
                print::print_node(&nodes);
                println!();
            }

            if configs {
                let config_dir = app_home.join("conf");

                if config_dir.exists() {
                    let mut entries = fs::read_dir(&config_dir).await?;

                    while let Some(entry) = entries.next_entry().await? {
                        let path = entry.path();
                        let conf_path = config_dir.join(path);
                        if conf_path.is_file() {
                            println!("(FrpcConf)\t{}", conf_path.display());
                        }
                    }
                } else {
                    return Err(anyhow!("[-] Conf dir not present!"));
                }
            }
        }
        Commands::Get { tunnel } => {
            let tunnel_node = chml
                .tunnel()
                .await?
                .into_result()?
                .iter()
                .find(|t| t.name.contains(&tunnel))
                .ok_or(anyhow!("{} not found", tunnel))?
                .node
                .clone();

            let tunnel_config = chml
                .tunnel_config(&tunnel_node, &[&tunnel])
                .await?
                .into_result()?;
            println!("{}", tunnel_config);
            println!();
        }
        Commands::Connect {
            tunnel,
            daemon,
            tunnel_id,
        } => {
            // 1. 总是拉取最新 tunnel 列表（用于多端同步）
            let the_tunnel = chml._select_tunnel(tunnel.as_deref(), tunnel_id).await?;

            let tunnel_name = &the_tunnel.name;
            let tunnel_node = the_tunnel.node.clone();

            let tunnel_config = chml
                .tunnel_config(&tunnel_node, &[tunnel_name])
                .await?
                .into_result()?;

            let config_path = app_home
                .join_dir("conf")?
                .join(format!("{}.frpc.ini", tunnel_name));

            fs::write(&config_path, tunnel_config).await?;

            if !cli.quiet {
                println!(
                    "[+] Tunnel({}) frpc config file has been downloaded at {}\n",
                    tunnel_name,
                    &config_path.display()
                );
            }

            let bin_path = app_home.join_dir("bin")?.join(config::bin_name());

            let mut child =
                spawn_frpc(&bin_path, &config_path, chml.get_token()?.to_owned()).await?;

            print::print_user_tunnels(&[the_tunnel.clone()]);
            child.wait().await.ok();
        }

        Commands::Add { resource } => match resource {
            // Add::Domain {
            //     record,
            //     r#type,
            //     ttl,
            //     rhost,
            // } => {
            //     // let cdp = CreateDomainParams{

            //     // };
            //     // let res = chml.create_free_subdomain(params);
            //     todo!()
            // }
            Add::Tunnel {
                name,
                r#type,
                lhost,
                lport,
                rport,
                node,
                china,
            } => {
                // 10000-65535 动态生成的话返回409则再次尝试
                let web = r#type.to_ascii_uppercase().contains("HTTP").then(|| true);
                let udp = r#type.to_ascii_uppercase().contains("UDP").then(|| true);

                let mut ctp = CreateTunnelParams {
                    token: chml.get_token()?.to_string(),
                    tunnelname: name.unwrap_or(format!("chml_{}", util::random_string(8))),
                    node: node
                        .unwrap_or(util::random_node(chml, web, udp, Some(china)).await?.name),
                    localip: lhost.unwrap_or("127.0.0.1".to_string()),
                    port_type: r#type.to_uppercase(),
                    local_port: lport,
                    encryption: false,
                    compression: false,
                    extra_params: "".to_string(),
                    remote_port: rport.unwrap_or(util::random_port()),
                };

                loop {
                    let tunnel = chml.create_tunnel(&ctp).await?;
                    if tunnel.code == 409 {
                        ctp.remote_port = util::random_port();
                        continue;
                    }

                    let tunnel = tunnel.into_result()?;
                    print::print_user_tunnels(&[tunnel]);
                    break;
                }
            }
        },
        Commands::Rm {
            tunnel_id,
            tunnel,
            // domain,
        } => {
            if let Some(tunnel_id) = tunnel_id {
                chml.delete_tunnel(&tunnel_id).await?;
                println!("[+] Tunnel {} has been deleted", tunnel_id);
            }

            if let Some(tunnel) = tunnel {
                let tunnels = chml.tunnel().await?.into_result()?;
                let tunnel_id = tunnels
                    .iter()
                    .find(|t| t.name.contains(&tunnel))
                    .ok_or(anyhow!("[-] {} doesn't existed", tunnel))?
                    .id
                    .ok_or(anyhow!("[-] {} has no id?", tunnel))?
                    .to_string();

                chml.delete_tunnel(&tunnel_id).await?;
                println!("[+] Tunnel {} has been deleted", tunnel);
            }

            // if let Some(domain) = domain {}
        }

        Commands::Tcp { port } => {
            let tunnel_name = format!("quick_chml_{}", util::random_string(4));
            let tunnel_node = util::random_node(chml, None, None, Some(true)).await?; // selectable
            let remote_port = util::random_port();
            let mut ctp = CreateTunnelParams {
                token: chml.get_token()?.to_string(),
                tunnelname: tunnel_name.clone(),
                node: tunnel_node.name.clone(),
                localip: "127.0.0.1".to_string(),
                port_type: "TCP".to_string(),
                local_port: port,
                encryption: false,
                compression: false,
                extra_params: "".to_string(),
                remote_port: remote_port,
            };

            loop {
                let tunnel = chml.create_tunnel(&ctp).await?;
                if tunnel.code == 409 {
                    ctp.remote_port = util::random_port();
                    continue;
                }
                break;
            }

            let tunnel_config = chml
                .tunnel_config(&tunnel_node.name, &[&tunnel_name])
                .await?
                .into_result()?;

            let config_path = app_home
                .join_dir("conf")?
                .join(format!("{}.frpc.ini", tunnel_name));

            fs::write(&config_path, tunnel_config).await?;

            let bin_path = app_home.join_dir("bin")?.join(config::bin_name());
            let tunnel = chml._select_tunnel(Some(&tunnel_name), None).await?;
            let tunnel_id = tunnel.id;
            print::print_user_tunnels(&[tunnel]);

            let mut child =
                spawn_frpc(&bin_path, &config_path, chml.get_token()?.to_owned()).await?;

            tokio::select! {
                _ = signal::ctrl_c() => {
                    eprintln!("\n[!] Ctrl+C received, shutting down frpc and remove{}, delete tunnel {}...", &config_path.display(), &tunnel_id.unwrap());

                    // 删除隧道和配置文件
                    let _ = tokio::fs::remove_file(&config_path).await;
                    let _ = chml.delete_tunnel(&tunnel_id.unwrap().to_string()).await;
                    let _ = child.kill().await;
                }

                status = child.wait() => {
                    eprintln!("[!] frpc exited: {:?}", status);
                }
            }
        }
        Commands::Http { port } => {
            unreachable!("Use tcp port")
        }
        Commands::Udp { port } => {
            let tunnel_name = format!("quick_chml_{}", util::random_string(4));
            let tunnel_node = util::random_node(chml, None, Some(true), Some(true)).await?; // selectable
            let remote_port = util::random_port();
            let mut ctp = CreateTunnelParams {
                token: chml.get_token()?.to_string(),
                tunnelname: tunnel_name.clone(),
                node: tunnel_node.name.clone(),
                localip: "127.0.0.1".to_string(),
                port_type: "UDP".to_string(),
                local_port: port,
                encryption: false,
                compression: false,
                extra_params: "".to_string(),
                remote_port: remote_port,
            };

            loop {
                let tunnel = chml.create_tunnel(&ctp).await?;
                if tunnel.code == 409 {
                    ctp.remote_port = util::random_port();
                    continue;
                }
                break;
            }

            let tunnel_config = chml
                .tunnel_config(&tunnel_node.name, &[&tunnel_name])
                .await?
                .into_result()?;

            let config_path = app_home
                .join_dir("conf")?
                .join(format!("{}.frpc.ini", tunnel_name));

            fs::write(&config_path, tunnel_config).await?;

            let bin_path = app_home.join_dir("bin")?.join(config::bin_name());
            let tunnel = chml._select_tunnel(Some(&tunnel_name), None).await?;
            let tunnel_id = tunnel.id;
            print::print_user_tunnels(&[tunnel]);

            let mut child =
                spawn_frpc(&bin_path, &config_path, chml.get_token()?.to_owned()).await?;

            tokio::select! {
                _ = signal::ctrl_c() => {
                    eprintln!("\n[!] Ctrl+C received, shutting down frpc and remove{}, delete tunnel {}...", &config_path.display(), &tunnel_id.unwrap());

                    // 删除隧道和配置文件
                    let _ = tokio::fs::remove_file(&config_path).await;
                    let _ = chml.delete_tunnel(&tunnel_id.unwrap().to_string()).await;
                    let _ = child.kill().await;
                }

                status = child.wait() => {
                    eprintln!("[!] frpc exited: {:?}", status);
                }
            }
        }
    }
    Ok(())
}

async fn spawn_frpc(
    bin_path: &std::path::Path,
    config_path: &std::path::Path,
    token: String,
) -> anyhow::Result<tokio::process::Child> {
    // token 脱敏函数
    fn mask_middle(s: &str) -> String {
        if s.len() <= 8 {
            "*".repeat(s.len())
        } else {
            let start = &s[..4];
            let end = &s[s.len() - 4..];
            let middle_len = s.len() - 8;
            format!("{}{}{}", start, "*".repeat(middle_len), end)
        }
    }

    let mut child = Command::new(bin_path)
        .arg("--config")
        .arg(config_path)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    // stdout
    if let Some(stdout) = child.stdout.take() {
        let token = token.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(mut line)) = reader.next_line().await {
                if line.contains(&token) {
                    line = line.replace(&token, &mask_middle(&token));
                }
                println!("[frpc stdout] {}", line);
            }
        });
    }

    // stderr
    if let Some(stderr) = child.stderr.take() {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Ok(Some(mut line)) = reader.next_line().await {
                if line.contains(&token) {
                    line = line.replace(&token, &mask_middle(&token));
                }
                eprintln!("[frpc stderr] {}", line);
            }
        });
    }

    Ok(child)
}
