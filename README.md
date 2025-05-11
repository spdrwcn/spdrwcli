```
DDR-SPD读写工具
支持通过串口与设备通信，读取和写入SPD数据，支持写保护解除设置

Usage: spdrw.exe <COMMAND>

Commands:
  list  列出所有可用串口
  send  向指定串口发送命令并接收响应 (如要查看该命令帮助，请运行 send -h)
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

使用示例:
  spdrw.exe list
  spdrw.exe send COM12 ddr4 read 0x01 0x02 12
```