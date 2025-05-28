# DDR-SPD 读写工具 使用文档

 - 本工具支持通过串口与设备通信，进行 DDR SPD 数据的 读取、写入、写保护设置/解除 等操作，同时也支持 WiFi 配置。

## 使用方式
```
spdrw.exe [子命令] [选项]
```

备注：选项均支持短选项，例如  `-p` 等同于 `--port-name`，wifi 密码短选项为`-w`.

### 子命令

#### list 列出可用端口/串口
```
spdrw.exe list
```

#### wifi 配置WiFi连接
```
spdrw.exe wifi --port-name COM1 --ssid "WiFi名称" --password "密码"
```

#### send SPD数据操作

 - 读取SPD数据  
```
spdrw.exe send --port-name COM1 --ddr-type ddr5 --cmd read --addr 00 -n 512
```

 - 写入SPD数据
```
spdrw.exe send --port-name COM1 --ddr-type ddr5 --cmd write --addr 00 --value AA
```

 - 设置SPD数据写保护
```
spdrw.exe send --port-name COM1 --ddr-type ddr5 --cmd srswp --value AABB
```

 - 解除SPD数据写保护
```
spdrw.exe send --port-name COM1 --ddr-type ddr5 --cmd crswp
```
