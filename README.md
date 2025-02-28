# 青鸾包管理器

## 使用说明

### 命令

```
用法：
QingLuanPackageManager [选项]
QingLuanPackageManager [子命令] [参数] [选项]

选项：
<-h | --help>            获取帮助
<-i | --init> [项目名称]  初始化项目
<-e>          [sdk版本]   sdk版本（在 -i 后）

子命令：
install     安装包
	[包名称] *[-v 包版本 | 默认: 最新]

uninstall   卸载包
	[包名称] -v [包版本]

find        查找包
	[包名称] -v [包版本]
```
