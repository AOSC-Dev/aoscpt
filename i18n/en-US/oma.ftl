# utils
can-not-run-dpkg-print-arch = Could not run `dpkg --print-architecture'.
execute-pkexec-fail = Failed to execute `pkexec': { $e }.
# history
history-tips-1 = oma has successfully applied changes to your system.
history-tips-2 = If you would like to undo these changes, please use the `oma undo' command.
# verify
fail-load-certs-from-file = Failed to load repository signature from { $path }.
cert-file-is-bad = Repository signature at { $path } is invalid.
# topics
can-not-find-specified-topic = Cannot find the specified topic repository: { $topic }.
do-not-edit-topic-sources-list = # Generated by oma. DO NOT EDIT!
select-topics-dialog = Enable topic(s) to receive testing updates, deselect to rollback to stable version(s):
tips = Press [Space]/[Enter] to toggle selection, [Esc] to apply changes, [Ctrl-c] to abort.
scan-topic-is-removed = Topic { $name } has been removed from the repository, disabling ...
refreshing-topic-metadata = Refreshing topics metadata ...
failed-to-read = Failed to read status file.
# pkg
can-not-get-pkg-from-database = Failed to get metadata for package { $name } from the local database.
debug-symbol-available = debug symbols available
full-match = exact match
invaild-path = Invaild path: { $p }
already-installed = { $name } ({ $version }) is already installed.
can-not-mark-reinstall = Cannot reinstall package { $name } ({ $version }), as the specified package and version could not be found in any available repository.
pkg-is-essential = Package { $name } is essential. Refusing to remove.
pkg-search-avail = AVAIL
pkg-search-installed = INSTALLED
pkg-search-upgrade = UPGRADE
pkg-no-checksum = Package { $name } no checksum.
pkg-unavailable = Version { $ver } of { $pkg } is not available from any configured repository.
# pager
question-tips-with-gui = Press [q] to end review, [Ctrl-c] to abort, [PgUp/Dn], arrow keys, or mouse wheel to scroll.
normal-tips-with-gui = Press [q] or [Ctrl-c] to exit, [PgUp/Dn], arrow keys, or mouse wheel to scroll.
question-tips = Press [q] to end review, [Ctrl-c] to abort, [PgUp/Dn] or arrow keys to scroll.
normal-tips = Press [q] or [Ctrl-c] to exit, [PgUp/Dn] or arrow keys to scroll.
# oma
no-need-to-do-anything = No need to do anything.
apt-error = `apt' returned an error.
invaild-pattern = Invalid pattern: { $p }
additional-version = { $len } additional version(s) available. Please use the `-a' switch to list all available version(s).
could-not-find-pkg-from-keyword = Could not find any package for keyword { $c }.
no-need-to-remove = Package { $name } is not installed. No need to remove.
packages-can-be-upgrade = { $len } package(s) can be upgraded.
packages-can-be-removed = { $len } package(s) can be removed.
comma = { ", " }
successfully-refresh-with-tips = Successfully refreshed the package database. { $s }
successfully-refresh = Successfully refreshed the package database. System is up to date.
no-candidate-ver = Current version for { $pkg } is not available from the repository.
pkg-is-not-installed = Unable to mark package { $pkg }, as it is not yet installed.
already-hold = Package { $name } is already marked for version hold.
set-to-hold = Marked package { $name } for version hold.
already-unhold = Package { $name } is not yet marked for version hold.
set-to-unhold = Marked package { $name } for version unhold.
already-manual = Package { $name } is already marked as manually installed.
setting-manual = Marked package { $name } as manually installed.
already-auto = Package { $name } is already marked as automatically installed.
setting-auto = Marked package { $name } as automatically installed.
command-not-found-with-result = { $kw }: command not found. The following package(s) may provide this command:
command-not-found = { $kw }: command not found.
clean-successfully = Successfully cleaned oma database and cache.
dpkg-configure-a-non-zero = `dpkg --configure -a' returned an error.
automatic-mode-warn = Running oma in unattended mode. If this is not intended, press Ctrl + C now to abort the operation!
removed-as-unneed-dep = removed as unneeded dependency
purge-file = purge configuration files
semicolon = { "; " }
pick-tips = Please select a version of { $pkgname } to install:
battery = You seem to be on battery power. oma may deplete the battery rather quickly during the transaction. It is recommended to plug in your power supply to prevent sudden power failure.
continue = Do you still wish to continue?
changing-system = oma is modifying your system.
failed-to-lock-oma = Another oma process lock (/run/lock/oma.lock) already exists
# main
user-aborted-op = User aborted the operation.
# formatter
count-pkg-has-desc = { $count } package(s) will be
dep-issue-1 = oma cannot install the specified package due to dependency issue(s).
dep-issue-2 = Below is a full dependency report, please copy or take a screenshot of the following and report it to an AOSC OS maintainer:
how-to-op-with-x = Press [PgUp/Dn], arrow keys, or use the mouse wheel to scroll.
end-review = Press [q] to end review
cc-to-abort = Press [Ctrl-c] to abort
how-to-op = Press [PgUp/Dn] or arrow keys to scroll.
total-download-size = { "Total download size: " }
change-storage-usage = { "Estimated change in storage usage: " }
pending-op = Pending Operations
review-msg =
    Shown below is an overview of the pending changes oma will apply to your
    system, please review them carefully.
install = install
installed = installed
remove = remove
removed = REMOVED
upgrade = upgrade
upgraded = upgraded
downgrade = downgrade
downgraded = downgraded
reinstall = reinstall
reinstalled = reinstalled
colon = : 

# download
invaild-filename = Invalid file name: { $name }.
checksum-mismatch-retry = Checksum verification failed for { $c }. Retrying { $retry } times ...
can-not-get-source-next-url = Failed to download { $e }. Retrying using the next available mirror ...
checksum-mismatch = Checksum verification failed for file { $filename }.
# db
invaild-url = Invalid URL { $url }.
can-not-parse-date = BUG: Failed to parse the Date field to the RFC2822 format. Please report this issue at https://github.com/AOSC-Dev/oma.
can-not-parse-valid-until = BUG: Failed to parse the Valid-Until field in the RFC2822 format. Please report this issue at https://github.com/AOSC-Dev/oma.
earlier-signature = InRelease file { $filename } is invalid: System time is earlier than the enclosed signature timestamp.
expired-signature = InRelease file { $filename } is invalid: The enclosed signature has already expired.
inrelease-sha256-empty = InRelease file is invalid: The SHA256 field is empty.
inrelease-checksum-can-not-parse = InRelease file is invalid: Failed to parse checksum entry { $i }.
inrelease-parse-unsupport-file-type = BUG: InRelease parser has encountered an unsupport file format. Please report this issue at https://github.com/AOSC-Dev/oma.
can-not-parse-sources-list = Failed to parse the sources.list file { $path }.
unsupport-protocol = oma does not support the protocol: { $url }.
refreshing-repo-metadata = Refreshing local database ...
not-found = Failed to download InRelease from { $url }: Remote file not found (404).
inrelease-syntax-error = InRelease file { $path } is invalid.
# contents
contents-does-not-exist = Package contents database (Contents) does not exist.
contents-may-not-be-accurate-1 = The local package contents database has not been updated for over a week, search results may not be accurate.
contents-may-not-be-accurate-2 = Use the `oma refresh' command to refresh the contents database.
execute-ripgrep-failed = Failed to execute `rg'.
searching = Searching ...
search-with-result-count = Searching, found { $count } results so far ...
contents-entry-missing-path-list = BUG: oma failed to parse the entry { $entry } in the local package contents database. Please report this issue at https://github.com/AOSC-Dev/oma.
rg-non-zero = `rg' returned an error.
# checksum
sha256-bad-length = Malformed SHA256 checksum: bad length.
can-not-checksum = Failed to parse SHA256 checksum.
failed-to-open-to-checksum = BUG: Failed to open { $path } for checksum verification. Please report this issue at https://github.com/AOSC-Dev/oma.
# config
config-invaild = oma configuration file appears to be broken (/etc/oma.toml)! Falling back to default configuration.
cleaning = Clearing packages cache ...
download-failed-with-len = { $len } package(s) failed to download.
download-failed = Failed to download { $filename }!
download-failed-no-name = Failed to download required file(s)!
need-more-size = Insufficient storage space: { $a } is availble, but { $n } is needed.
successfully-download-to-path = Successfully downloaded { $len } package(s) to path: { $path }.
oma-may =
    oma may { $a }, { $b }, { $c }, { $d }, or { $e } packages in order
    to fulfill your requested changes.
failed-to-read-decode-inrelease = Failed to read the decoded InRelease file.
failed-to-operate-path = Failed to perform file operations in { $p }.
failed-to-get-parent-path = Failed to get parent path of { $p }.
failed-to-read-file-metadata = Failed to read file metadata for { $p }.
failed-to-get-rg-process-info = Failed to get process status for `rg'.
failed-to-calculate-available-space = Failed to calculate available storage space.
failed-to-create-http-client = Failed to create an HTTP client.
failed-to-connect-history-database = Failed to connect to the history database.
failed-to-execute-query-stmt = Failed to query the history database.
failed-to-parse-history-object = Failed to parse an object in the history database.

failed-to-set-lockscreen = Failed to set system lockscreen status.
failed-to-create-proxy = Failed to create a system message bus (D-Bus) proxy: { $proxy }.

failed-check-dbus = oma is unable to detect the status of the currently managed system because it has not yet started up.
failed-check-dbus-tips-1 = In this state, oma is unable to check the status of critical systems such as power supplies and user sessions, and continued operation may result in system failure!
failed-check-dbus-tips-2 = If you are trying to repair the system using, for example, chroot, please add the --no-check-dbus parameter after the oma command.
failed-check-dbus-tips-3 = If your system is permanently running in a container or chroot environment, modify the value of the `no_check_dbus' option in oma's configuration file (/etc/oma.toml) to `true'.
no-check-dbus-tips = oma is configured to ignore system operational status, which means that oma will not detect critical system states such as power supplies and user sessions. System failures may occur.
oma-history-is-empty = oma history is empty.
tui-pending = Pending
tui-search = Search
tui-packages = Package(s) ({ $u } upgradable, has { $r } removable, { $i } installed)
tui-start-1 = Welcome to oma!
tui-start-2 = Switch window
tui-start-3 = Hide/Show pending operations
tui-start-4 = Apply changes
tui-start-5 = Add/Remove item(s)
tui-start-6 = Search
tui-start-7 = Exit
tui-start = OK
oma = oma
another-oma-is-running = Another oma instance is running: { $s }
table-name = Name
table-version = Version
table-size = Installed size
table-detail = Detail
reading-database = Reading packages database
has-error-on-top = The operation you specified did not complete successfully. Please refer to the output above for details.
