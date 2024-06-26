use std::str::FromStr;

use jni::errors::Error;
use jni::objects::JObject;
use jni::JNIEnv;

use vnt::channel::punch::PunchModel;
use vnt::channel::UseChannelType;
use vnt::cipher::CipherModel;
use vnt::core::Config;

use crate::utils::*;

pub fn new_config(env: &mut JNIEnv, config: JObject) -> Result<Config, Error> {
    #[cfg(target_os = "windows")]
    let tap = env.get_field(&config, "tap", "Z")?.z()?;
    let token = to_string_not_null(env, &config, "token")?;
    let name = to_string_not_null(env, &config, "name")?;
    let device_id = to_string_not_null(env, &config, "deviceId")?;
    let password = to_string(env, &config, "password")?;
    let server_address_str = to_string_not_null(env, &config, "server")?;
    let stun_server = to_string_array_not_null(env, &config, "stunServer")?;
    let dns = to_string_array(env, &config, "dns")?.unwrap_or_else(|| vec![]);
    let port_mapping = to_string_array(env, &config, "portMapping")?.unwrap_or_else(|| vec![]);
    let cipher_model = to_string_not_null(env, &config, "cipherModel")?;
    let punch_model = to_string(env, &config, "punchModel")?;
    let mtu = to_integer(env, &config, "mtu")?.map(|v| v as u32);
    let tcp = env.get_field(&config, "tcp", "Z")?.z()?;
    let server_encrypt = env.get_field(&config, "serverEncrypt", "Z")?.z()?;
    let use_channel = to_string(env, &config, "useChannel")?;
    let finger = env.get_field(&config, "finger", "Z")?.z()?;
    let first_latency = env.get_field(&config, "firstLatency", "Z")?.z()?;
    let packet_delay = to_integer(env, &config, "packetDelay")?
        .map(|v| v as u32)
        .unwrap_or_default();
    let packet_loss_rate = to_double(env, &config, "packetLossRate")?;

    let in_ips = to_string_array(env, &config, "inIps")?;
    let out_ips = to_string_array(env, &config, "outIps")?;
    let ports =
        to_i32_array(env, &config, "ports")?.map(|v| v.into_iter().map(|v| v as u16).collect());
    let ip = if let Some(ip) = to_string(env, &config, "ip")? {
        match ip.parse() {
            Ok(ip) => Some(ip),
            Err(e) => {
                env.throw_new(
                    "java/lang/RuntimeException",
                    format!("ip {} err: {}", ip, e),
                )
                .expect("throw");
                return Err(Error::JavaException);
            }
        }
    } else {
        None
    };
    let in_ips = if let Some(in_ips) = in_ips {
        match common::args_parse::ips_parse(&in_ips) {
            Ok(in_ips) => in_ips,
            Err(e) => {
                env.throw_new("java/lang/RuntimeException", format!("in_ips {}", e))
                    .expect("throw");
                return Err(Error::JavaException);
            }
        }
    } else {
        vec![]
    };
    let out_ips = if let Some(out_ips) = out_ips {
        match common::args_parse::out_ips_parse(&out_ips) {
            Ok(out_ips) => out_ips,
            Err(e) => {
                env.throw_new("java/lang/RuntimeException", format!("out_ips {}", e))
                    .expect("throw");
                return Err(Error::JavaException);
            }
        }
    } else {
        vec![]
    };

    let cipher_model = match CipherModel::from_str(&cipher_model) {
        Ok(cipher_model) => cipher_model,
        Err(e) => {
            env.throw_new("java/lang/RuntimeException", format!("cipher_model {}", e))
                .expect("throw");
            return Err(Error::JavaException);
        }
    };
    #[cfg(not(target_os = "android"))]
    let device_name = to_string(env, &config, "deviceName")?;
    let config = match Config::new(
        #[cfg(target_os = "windows")]
        tap,
        token,
        device_id,
        name,
        server_address_str,
        dns,
        stun_server,
        in_ips,
        out_ips,
        password,
        mtu,
        tcp,
        ip,
        false,
        server_encrypt,
        1,
        cipher_model,
        finger,
        PunchModel::from_str(&punch_model.unwrap_or_default()).unwrap_or_default(),
        ports,
        first_latency,
        #[cfg(not(target_os = "android"))]
        device_name,
        UseChannelType::from_str(&use_channel.unwrap_or_default()).unwrap_or_default(),
        packet_loss_rate,
        packet_delay,
        port_mapping,
    ) {
        Ok(config) => config,
        Err(e) => {
            env.throw_new(
                "java/lang/RuntimeException",
                format!("vnt start error {:?}", e),
            )
            .expect("throw");
            return Err(Error::JavaException);
        }
    };
    Ok(config)
}
