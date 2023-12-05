# utils
can-not-run-dpkg-print-arch = 无法执行 `dpkg --print-architecture'。
execute-pkexec-fail = 无法执行 `pkexec' 命令：{$e}。

# history
history-tips-1 = Omakase 已成功应用对系统的更改。
history-tips-2 = 如需撤销本次操作，请使用 `oma undo' 命令。

# verify
fail-load-certs-from-file = 无法从 {$path} 载入软件源签名。
cert-file-is-bad = 位于 {$path} 的软件源签名无效。

# topics
can-not-find-specified-topic = 找不到测试源：{$topic}
do-not-edit-topic-sources-list = # 本文件使用 Omakase 生成，请勿编辑！
select-topics-dialog = 打开测试源以获取实验性更新，关闭测试源以回滚到稳定版本：
tips = 按 [Space]/[Enter] 开关测试源，按 [Esc] 应用更改，按 [Ctrl-c] 退出。
scan-topic-is-removed = 测试源 {$name} 已从软件源中被删除，现将关闭 ...
refreshing-topic-metadata = 正在刷新测试源数据 ...
failed-to-read = 无法读取状态文件。

# pkg
can-not-get-pkg-from-database = 无法从本机数据库中获取软件包 {$name} 的元数据。
invaild-path = 非法路径：{$p}
debug-symbol-available = 调试符号可用
full-match = 完整匹配
already-installed = 软件包 {$name} ({$version}) 已经安装。
can-not-mark-reinstall = 无法重装软件包 {$name} ({$version})，因为当前可用的软件源中找不到指定的软件包和版本。
pkg-is-essential = 软件包 {$name} 是不允许删除的关键组件。
pkg-search-avail = AVAIL
pkg-search-installed = INSTALLED
pkg-search-upgrade = UPGRADE
pkg-no-checksum = 软件包 {$name} 没有校验码。
pkg-unavailable = 软件源中找不到 {$ver} 版本的软件包 {$pkg}。

# pager
question-tips-with-x11 = 按 [q] 结束审阅并应用更改，按 [Ctrl-c] 中止操作，按 [PgUp/Dn]、方向键或使用鼠标滚轮翻页。
normal-tips-with-x11 = 按 [q] 或 [Ctrl-c] 退出，按 [PgUp/Dn]、方向键或使用鼠标滚轮翻页。
question-tips = 按 [q] 结束审阅并应用更改，按 [Ctrl-c] 中止操作，按 [PgUp/Dn] 或方向键翻页。
normal-tips = 按 [q] 或 [Ctrl-c] 退出，按 [PgUp/Dn] 或方向键翻页。

# oma
no-need-to-do-anything = 无需进行任何操作。
apt-error = `apt' 返回了错误。
invaild-pattern = 非法的表达式：{$p}
additional-version = 另有 {$len} 个可用版本。请使用 `-a' 列出所有可用版本。
could-not-find-pkg-from-keyword = 无法找到匹配关键字 {$c} 的软件包
no-need-to-remove = 软件包 {$name} 尚未安装，因此无需卸载。
packages-can-be-upgrade = 有 {$len} 个可升级的软件包。
packages-can-be-removed = 有 {$len} 个可删除的软件包。
comma = ，
successfully-refresh-with-tips = 成功刷新本机软件包数据库。{$s}
successfully-refresh = 成功刷新本机软件包数据库。系统各软件包均为最新。
no-candidate-ver = 无法从软件包仓库中获取当前版本的软件包 {$pkg} 。
pkg-is-not-installed = 无法标记软件包 {$pkg} 的属性，因为该软件包尚未安装。
already-hold = 软件包 {$name} 已被标记为版本锁定。
set-to-hold = 成功标记软件包 {$name} 属性：版本锁定。
already-unhold = 软件包 {$name} 尚未标记为版本锁定。
set-to-unhold = 成功标记软件包 {$name} 属性：版本解锁。
already-manual = 软件包 {$name} 已被标记为手动安装。
setting-manual = 成功标记软件包 {$name} 属性：手动安装。
already-auto = 软件包 {$name} 已被标记为自动安装。
setting-auto = 成功标记软件包 {$name} 属性：自动安装。
command-not-found-with-result = {$kw}：找不到命令。该命令由如下软件包提供：
command-not-found = {$kw}：找不到命令。
clean-successfully = 成功清理 Omakase 本机数据库和缓存。
dpkg-configure-a-non-zero = `dpkg --configure -a' 返回错误。
automatic-mode-warn = 正以无人值守模式运行 Omakase。如非本人所为，请立即按 Ctrl + C 中止操作！
removed-as-unneed-dep = 清理未使用的依赖
purge-file = 清理配置文件
semicolon = ；
pick-tips = 请指定软件包 {$pkgname} 的版本：
battery = 您的电脑目前似乎正在使用电池供电。oma 在执行任务时可能会消耗大量电量，推荐您接入交流电源以防断电导致数据损坏。
continue = 您确定要继续吗？
changing-system = oma 正在修改你的系统

# main
user-aborted-op = 用户已中止操作。

# formatter
count-pkg-has-desc = {$count} 个软件包将被
dep-error = 依赖关系错误
dep-error-desc = Omakase 探测到依赖关系问题，因此无法继续进行指定操作。该问题可能是
    软件包缺失或来源不统一，抑或是指定的软件包版本与当前系统不兼容导致的。
contact-admin-tips = 请联系您的系统管理员或开发者。
how-to-abort = 按 [q] 或 [Ctrl-c] 中止操作
how-to-op-with-x = 按 [PgUp/Dn]、方向键或使用鼠标滚轮翻页
end-review = 按 [q] 结束审阅并应用更改
cc-to-abort = 按 [Ctrl-c] 中止操作
how-to-op = 按 [PgUp/Dn] 或方向键翻页
total-download-size = {"总下载大小： "}
change-storage-usage = {"预计磁盘占用变化： "}
pending-op = 待操作清单
review-msg = Omakase 将执行如下操作，请仔细验证。
install = 安装
installed = 安装
remove = 卸载
removed = 卸载
upgrade = 更新
upgraded = 更新
downgrade = 降级
downgraded = 降级
reinstall = 重装
reinstalled = 重装
unmet-dep = 无法满足
colon = ：
unmet-dep-before = 有 {$count} 个软件包的依赖

# download
invaild-filename = 文件名 {$name} 无效。
checksum-mismatch-retry = 文件 {$c} 完整性验证失败，将重试 {$retry} 次 ...
can-not-get-source-next-url = 无法下载文件：{$e}，将使用下一个镜像源重试 ...
checksum-mismatch = 文件 {$filename} 完整性验证失败。

# db
invaild-url= URL {$url} 无效
can-not-parse-date = BUG：无法将 Date 值转换为 RFC2822 格式，请于 https://github.com/AOSC-Dev/oma 报告问题。
can-not-parse-valid-until = BUG：无法将 Valid-Until 值转换为 RFC2822 格式，请于 https://github.com/AOSC-Dev/oma 报告问题。
earlier-signature = InRelease 文件 {$filename} 无效：系统时间早于内联签名时间戳。
expired-signature = InRelease 文件 {$filename} 无效：内联签名已过期。
inrelease-sha256-empty = InRelease 文件无效：SHA256 值为空。
inrelease-checksum-can-not-parse = InRelease 文件无效：无法解析校验和条目 {$i}。
inrelease-parse-unsupport-file-type = BUG：解析器不支持该 InRelease 文件的格式，请于 https://github.com/AOSC-Dev/oma 报告问题。
can-not-parse-sources-list = 无法解析 sources.list 文件 {path}。
unsupport-protocol = Omakase 不支持协议：{$url}。
refreshing-repo-metadata = 正在刷新本机软件包数据库 ...
not-found = 无法从 {$url} 下载 InRelease 文件：找不到远端文件 (HTTP 404)。
inrelease-syntax-error = 位于 {$path} 的 InRelease 文件解析失败。

# contents
contents-does-not-exist = 找不到软件包内容数据库文件 (Contents)。
contents-may-not-be-accurate-1 = 本机软件包内容数据库文件已超过一周未有更新，因此搜索结果可能不准确。
contents-may-not-be-accurate-2 = 请使用 `oma refresh' 命令刷新该数据库。
execute-ripgrep-failed = 无法执行 `rg' 命令。
searching = 正在搜索 ...
search-with-result-count = 正在搜索，已找到 {$count} 个结果 ...
contents-entry-missing-path-list = BUG：Omakase 无法解析本机软件包内容数据库中的条目 {$entry}，请于 https://github.com/AOSC-Dev/oma 报告问题。
rg-non-zero = `rg' 报错退出。

# checksum
sha256-bad-length = SHA256 校验和无效：长度不正确。
can-not-checksum = 无法解析 SHA256 校验和。
failed-to-open-to-checksum = BUG：无法打开用于验证校验和的路径 {$path}，请于 https://github.com/AOSC-Dev/oma 报告问题。

# config
config-invaild = Omakase 配置文件 (/etc/oma.toml) 似乎已损坏！将使用默认配置。
cleaning = 正在清理本地软件包缓存 ...
download-failed-with-len = {$len} 个软件包下载失败。
download-failed = 下载 {$filename} 文件失败！
download-failed-no-name = 下载文件失败！

need-more-size = 存储空间不足：{$a} 可用，但需要 {$n}。
successfully-download-to-path = 已下载 {$len} 个软件包到该路径：{$path}
oma-may = 为应用您指定的更改，Omakase 可能 {$a}、{$b}、{$c}、{$d} 或 {$e} 软件包。

failed-to-read-decode-inrelease = 无法读取解密后的 InRelease 文件。
failed-to-operate-path = 无法在路径 {$p} 中执行文件操作。
failed-to-get-parent-path = 无法获取路径 {$p} 的父目录。
failed-to-read-file-metadata = 无法读取 {$p} 的文件元数据。
failed-to-get-rg-process-info = 无法获取 `rg' 的进程状态。
failed-to-calculate-available-space = 无法计算可用存储空间。
failed-to-create-http-client = 无法创建 HTTP 客户端。
failed-to-connect-history-database = 无法连接到历史数据库。
failed-to-execute-query-stmt = 无法在历史数据库中执行查询命令。
failed-to-parse-history-object = 无法解析历史数据库中的对象。

failed-to-connect-dbus = 无法连接到系统消息总线 (D-Bus) 。
failed-to-set-lockscreen = 无法设置系统锁屏状态。
failed-to-create-proxy = 无法创建系统消息总线 (D-Bus) 代理：{$proxy}。
failed-to-get-power-status = 无法获取设备供电状态。