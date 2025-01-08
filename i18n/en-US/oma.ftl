# utils
can-not-run-dpkg-print-arch = Could not run `dpkg --print-architecture'.
execute-pkexec-fail = Failed to execute `pkexec': { $e }.
# history
history-tips-1 = oma has successfully applied changes to your system.
history-tips-2 = If you would like to undo these changes, please use the { $cmd } command.
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
failed-to-read = Failed to read status file ({ $p }).
# pkg
can-not-get-pkg-from-database = Failed to get metadata for package { $name } from the local database.
debug-symbol-available = debug symbols available
full-match = exact match
invalid-path = Invalid path: { $p }
already-installed = { $name } ({ $version }) is already installed.
can-not-mark-reinstall = Cannot reinstall package { $name } ({ $version }), as the specified package and version could not be found in any available repository.
pkg-is-essential = Package { $name } is essential. Refusing to remove.
pkg-no-checksum = oma is unable to repair { $name }: unable find checksum for this local or 3rd-party package.
pkg-unavailable = Version { $ver } of { $pkg } is not available from any configured repository.
# pager
question-tips-with-gui = Press [q] to end review, [Ctrl-c] to abort, [PgUp/Dn], arrow keys, or mouse wheel to scroll.
normal-tips-with-gui = Press [q] or [Ctrl-c] to exit, [PgUp/Dn], arrow keys, or mouse wheel to scroll.
question-tips = Press [q] to end review, [Ctrl-c] to abort, [PgUp/Dn] or arrow keys to scroll.
normal-tips = Press [q] or [Ctrl-c] to exit, [PgUp/Dn] or arrow keys to scroll.
# oma
no-need-to-do-anything = No need to do anything.
apt-error = `apt' returned an error.
invalid-pattern = Invalid pattern: { $p }
additional-version = { $len } additional version(s) available. Please use the `-a' switch to list all available version(s).
could-not-find-pkg-from-keyword = Could not find any package for keyword { $c }.
no-need-to-remove = Package { $name } is not installed. No need to remove.
packages-can-be-upgrade = { $len } package(s) can be upgraded
packages-can-be-removed = { $len } package(s) can be removed
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
dpkg-triggers-only-a-non-zero = `dpkg --triggers-only -a' returned an error.
automatic-mode-warn = Running oma in unattended mode. If this is not intended, press Ctrl + C now to abort the operation!
removed-as-unneed-dep = Removed as unneeded dependency
removed-as-unmet-dep = Removed due to unmet dependency
purge-file = Purge configuration files
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
dep-issue-2 = Please copy or take a screenshot of the following and report it to an AOSC OS maintainer:
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
invalid-filename = Invalid file name: { $name }.
checksum-mismatch-retry = Checksum verification failed for { $c }. Retrying { $retry } times ...
can-not-get-source-next-url = Failed to download { $e }. Retrying using the next available mirror ...
checksum-mismatch = Checksum verification failed for file { $filename }.
# db
invalid-url = Invalid URL { $url }.
can-not-parse-date = BUG: Failed to parse the Date field to the RFC2822 format. Please report this issue at https://github.com/AOSC-Dev/oma.
can-not-parse-valid-until = BUG: Failed to parse the Valid-Until field in the RFC2822 format. Please report this issue at https://github.com/AOSC-Dev/oma.
earlier-signature = InRelease file { $filename } is invalid: System time is earlier than the enclosed signature timestamp.
expired-signature = InRelease file { $filename } is invalid: The enclosed signature has already expired.
inrelease-sha256-empty = No checksums value available in InRelease.
inrelease-checksum-can-not-parse = InRelease file is invalid: Failed to parse file { $p }.
inrelease-parse-unsupported-file-type = BUG: InRelease parser has encountered an unsupported file format. Please report this issue at https://github.com/AOSC-Dev/oma.
can-not-parse-sources-list = Failed to parse the sources.list file { $path }.
unsupported-protocol = oma does not support the protocol: { $url }.
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
config-invalid = oma configuration file appears to be broken (/etc/oma.toml)! Falling back to default configuration.
cleaning = Clearing packages cache ...
download-failed-with-len = { $len } package(s) failed to download.
download-failed = Failed to download { $filename }!
download-failed-no-name = Failed to download required file(s)!
need-more-size = Insufficient storage space: { $a } is available, but { $n } is needed.
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
tui-upgrade = Upgrade system
tui-autoremove = Clean package(s)
tui-no-system-update = No system update available at this time.
tui-no-package-clean-up = No package clean-up is required at this time.
tui-continue-tips = Press [c] to continue.
oma = oma
another-oma-is-running = Another oma instance is running: { $s }
table-name = Name
table-version = Version
table-size = Installed size
table-detail = Detail
reading-database = Building index for the package database ...
reading-database-with-count = Building index for the package database, { $count } package(s) indexed ...
has-error-on-top = The operation you specified did not complete successfully. Please refer to the output above for details.
mirror-is-not-trusted = Mirror { $mirror } is unsigned or not trusted, please check your repository settings.
please-run-me-as-root = oma requires elevated privileges to perform system administration.
topic-not-in-mirror = Topic { $topic } is inaccessible in mirror { $mirror }.
skip-write-mirror = This is likely because the mirror(s) you specified are not yet synchronized. Skipping this mirror in favor of other options ...
failed-to-decompress-contents = Failed to decompress contents.
pkexec-tips-1 = Please input your password in the password prompt to grant oma privileges to perform system administration.
pkexec-tips-2 = If you are unable to access the graphical prompt, please press Ctrl + C to abort and run oma with elevated privileges.
cnf-too-many-query = It seems that there are many matches for the command you are looking for.
cnf-too-many-query-2 = Please consider using `oma provides --bin { $query }' for a full list of potential matches.
no-result-bincontents-tips = You might be looking for:
no-result-bincontents-tips-2 = The { $cmd } command comes from the package { $pkg }
oma-refresh-lock = Unable to refresh local database.
oma-refresh-lock-dueto = { $exec } ({ $pid }) has acquired the lock to the database.
oma-refresh-success-invoke = Executing Post-refresh configuration script (Post-Invoke-Success) ...
autoremove-tips-1 = { $count } unneeded packages on your system may be removed, which will free up { $size } in storage space; Please use { $cmd } to view the list of packages that can be removed.
autoremove-tips-2 = If you would like to keep a particular package, use { $cmd1 } to mark the package as manually installed; Otherwise, you may use { $cmd2 } to clean up the packages that are no longer needed.
essential-tips = { $pkg } is an ESSENTIAL system component, removing it will render the system inoperable.
essential-continue = Are you sure that you would like to remove it?
yes-do-as-i-say-prompt = Your turn
yes-do-as-i-say = If you are absolutely sure, please type the following: { $input }
yes-do-as-i-say-abort = Prompt answered incorrectly.
features-without-value = The current operation would remove key AOSC OS components. If we proceed, Will cause some system features to be unavailable.
features-tips-1 = The current operation would remove key AOSC OS components. If we proceed, the system features below will no longer be available:
features-abort = To avoid system failure, oma has aborted the operation.
features-continue-prompt = Would you like to proceed with the operation?
select-mirror-prompt = Press [Space]/[Enter] to toggle a mirror on or off
set-mirror-order-prompt = Use [Space] to select a mirror, then up/down arrows to re-order
all-speedtest-failed = All mirror speedtests failed, please check your Internet connection.
failed-to-parse-file = Failed to parse file { $p }.
mirror-not-found = Unable to find mirror { $mirror }. Please use `oma mirror' and select from a list of available mirrors.
failed-to-serialize-struct = BUG: Unexpected error while parsing oma configuration.
failed-to-write-file = Unable to write to { $p }.
failed-to-create-file = Unable to create { $p }.
mirror-name = Mirror
mirror-score = Score
mirror-speedtest-start = Commencing mirror benchmark ...
speedtest-complete = Mirror benchmark complete.
search-tips-with-result = Press [Esc] to cancel search, [N]/[n] to jump to the previous or next match.
search-tips-with-query = Search: { $query }
search-tips-with-empty = The search pattern may not be empty (press [/] to retry).
search-tips-not-found = Your search did not return any match (press [/] to retry).
doplicate-component = Duplicate sources entries found in source configuration: { $url } { $c }.
resolving-dependencies = Resolving dependencies ...
auth-need-permission = The repository shipping the package(s) may require authorization. Please run oma with elevated privileges, as it needs to read the authorization configurations.
lack-auth-config-1 = oma is unable to access the specified package(s) from the repository due to missing authorization configurations. 
lack-auth-config-2 = Please verify if your APT authorization configurations (/etc/apt/auth.conf.d) were set up correctly.
success = The operation was completed successfully.
topics-unchanged = oma was unable complete topic opt-out operation(s) and has kept the topic status unchanged.
status-pending = Pending
status-downloading = Downloading
status-package = Processing package { $pkg }
update-available = System Update(s) Available
update-available-2 = oma has found the following updates are available for your system:
tum-1 = oma has found { $updates } updates available for your system.
tum-1-with-security =  oma has found { $updates } updates available for your system ({ $security } security updates are marked in { $security_str }).
tum-2 = In order to fulfill the operations you specified, oma will need to perform the following changes to your system components. The details are as follows.Details of the component changes for this operation are as follows.
security = security update(s)
verify-error = An error occurred while verifying the signature for { $p }.
unsupported-sources-list = Unsupported source list files: { $p }, oma only line style sources list (.list) and deb822 style source list (.sources) are supported.
