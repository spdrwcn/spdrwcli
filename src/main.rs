use clap::{Parser, Subcommand};
use serde_json::json;
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::io::{self, Read, Write};
use std::time::Duration;
use serialport::available_ports;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "DDR-SPD读写工具\n支持通过串口与设备通信，读取和写入SPD数据，支持写保护“解除” “设置”",
    long_about = None,
    after_help = "使用示例:\n  spdrw.exe list\n  spdrw.exe send COM12 ddr4 read 0x01 0x02 12"
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 列出所有可用串口
    List,
    /// 向指定串口发送命令并接收响应 (如要查看该命令帮助，请运行 send -h)
    Send {
        /// 串口名称 (如 COM3 或 /dev/ttyUSB0)
        #[arg(verbatim_doc_comment, required = true)]
        port_name: String,
        /// type: DDR内存类型，可选值：ddr4、ddr5
        #[arg(verbatim_doc_comment, required = true)]
        ddr_type: String,
        /// cmd: 操作指令，可选值：read、write、srswp、crswp
        #[arg(verbatim_doc_comment, required = true)]
        cmd: String,
        /// addr: 操作的字节地址，以十六进制表示（如 "0x00"），仅在 read write 中可用
        #[arg(verbatim_doc_comment, default_value = "0x00")]
        addr: String,
        /// value: 写入的值，以十六进制表示（如 "0x00"），仅在 write srswp中可用
        #[arg(verbatim_doc_comment, default_value = "0x00")]
        value: String,
        /// number: 读取的字节数，以十进制表示（如 "128"），仅在 read 中可用
        #[arg(verbatim_doc_comment, default_value = "1")]
        number: String,
    },
}

fn list_ports() -> io::Result<()> {
    let ports = available_ports().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    if ports.is_empty() {
        println!("无可用端口");
    } else {
        for port in ports {
            println!("{}", port.port_name);
        }
    }
    Ok(())
}

fn format_command(
    ddr_type: &str,
    cmd: &str,
    addr: &str,
    value: &str,
    number: &str,
) -> String {
    let command = json!({
        "type": ddr_type,
        "cmd": cmd,
        "addr": addr,
        "value": value,
        "number": number
    });
    command.to_string() + "\n"
}

fn send_command(port_name: &str, command: &str) -> io::Result<()> {
    let mut port = serialport::new(port_name, 115200)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .parity(Parity::None)
        .flow_control(FlowControl::None)
        .timeout(Duration::from_millis(100)) 
        .open()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    port.write_all(command.as_bytes())?;
    port.flush()?;

    let mut buffer = vec![0; 4096];
    let mut total_bytes_read = 0;
    loop {
        match port.read(&mut buffer[total_bytes_read..]) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    total_bytes_read += bytes_read;
                    if buffer[..total_bytes_read].contains(&b'\n') {
                        break;
                    }
                } else {
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
            Err(e) => return Err(e),
        }
    };
    

    let received_data = String::from_utf8_lossy(&buffer[..total_bytes_read]);

    if total_bytes_read == 1026 || total_bytes_read == 2050 {
        for (i, chunk) in received_data.as_bytes().chunks(32).enumerate() {
            let hex_str = std::str::from_utf8(chunk).unwrap();
            print!("0x{:04x}: ", i * 16);
            for j in 0..16 {
                let pos = j * 2;
                if pos + 2 <= hex_str.len() {
                    print!("{} ", &hex_str[pos..pos + 2]);
                }
            }
            println!();
        }
    } else {
        println!("{}", received_data);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::List => list_ports(),
        Commands::Send {
            port_name,
            ddr_type,
            cmd,
            addr,
            value,
            number,
        } => {
            let command = format_command(&ddr_type, &cmd, &addr, &value, &number);
            send_command(&port_name, &command)
        },
    }
}