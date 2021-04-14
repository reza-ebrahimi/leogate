use async_graphql::*;

use futures::lock::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

use systemstat::{data, Platform, System};

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
struct OsInfo {
  os_type: String,
  version: String,
  edition: String,
  codename: String,
}

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
struct CpuLoad {
  user: f32,
  nice: f32,
  system: f32,
  interrupt: f32,
  idle: f32,
}

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
struct ServerInfo {
  cpu_temp: f32,
  boot_time: String,
  uptime: f64,
  on_ac_power: bool,
}

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
struct Networks {
  interfaces: HashMap<String, Vec<String>>,
}

#[derive(Clone, Default)]
pub struct SystemStatusQuery;

#[Object]
impl SystemStatusQuery {
  async fn server_info(&self) -> Option<ServerInfo> {
    let sys = System::new();
    Some(ServerInfo {
      cpu_temp: sys.cpu_temp().unwrap_or_default(),
      boot_time: sys
        .boot_time()
        .unwrap()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string(),
      uptime: sys.uptime().unwrap().as_secs_f64(),
      on_ac_power: sys.on_ac_power().unwrap_or_default(),
    })
  }

  async fn os_info(&self) -> Option<OsInfo> {
    let info = os_info::get();
    Some(
      OsInfo {
        os_type: info.os_type().to_string(),
        version: info.version().to_string(),
        edition: info.edition().unwrap_or_default().to_string(),
        codename: info.codename().unwrap_or_default().to_string(),
      }
      .into(),
    )
  }

  async fn networks(&self) -> Option<Networks> {
    let sys = System::new();
    let mut interfaces: HashMap<String, Vec<String>> = HashMap::new();
    for netif in sys.networks().unwrap().values() {
      let mut addresses: Vec<String> = vec![];
      for addr in netif.addrs.iter() {
        match addr.addr {
          data::IpAddr::Empty => {}
          data::IpAddr::Unsupported => {}
          data::IpAddr::V4(inner) => {
            addresses.push(inner.to_string());
          }
          data::IpAddr::V6(inner) => {
            addresses.push(inner.to_string());
          }
        }
      }
      interfaces.insert(String::from(netif.name.as_str()), addresses);
    }
    Some(Networks { interfaces }.into())
  }

  async fn cpu_load(&self, ctx: &Context<'_>) -> Option<CpuLoad> {
    ctx
      .data_unchecked::<Arc<Mutex<tokio::runtime::Runtime>>>()
      .lock()
      .await
      .spawn(async {
        let sys = System::new();
        let aggregate = sys.cpu_load_aggregate();

        tokio::time::sleep(std::time::Duration::from_millis(250)).await;

        match aggregate {
          Ok(cpu) => {
            let cpu = cpu.done().unwrap();
            Some(
              CpuLoad {
                user: cpu.user * 100.0,
                nice: cpu.nice * 100.0,
                system: cpu.system * 100.0,
                interrupt: cpu.interrupt * 100.0,
                idle: cpu.idle * 100.0,
              }
              .into(),
            )
          }
          Err(x) => {
            println!("\nCPU load: error: {}", x);
            None
          }
        }
      })
      .await
      .unwrap()
  }
}
