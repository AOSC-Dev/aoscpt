# utils
can-not-run-dpkg-print-arch = Can not run dpkg, why: {$e}
dpkg-return-non-zero = dpkg return non-zero code: {$e}
need-more-size = Your disk space is too small, need size: {$n}, available space: {$a}
old-pid-still-running = Another instance of oma (pid: {$pid}) is still running!
can-not-create-lock-dir = Can not create /run/lock dir! why: {$e}
can-not-create-lock-file = Can not create lock file! why: {$e}
can-not-write-lock-file = Can not write lock file! why: {$e}
can-not-unlock-oma = Can not unlock oma! why: {$e}
can-not-create-oma-log-dir = Can not create oma log directory, why: {$e}
can-not-create-oma-log = Can not create oma history file, why: {$e}
execute-pkexec-fail = Execute pkexec failed, why: {$e}

# verify
fail-load-certs-from-file = Failed to load certs from file {$path}
cert-file-is-bad = A cert from file {$path} is bad
inrelease-bad-signature = InRelease contains bad signature: {$e} 
inrelease-must-signed = Malformed PGP signature, InRelease must be signed.

# topics
can-not-find-specified-topic = Cannot find the specified topic: {$topic}
failed-to-enable-following-topics = Failed to enable the following topics: {$topic}
saving-topic-settings = Saving topic enrollment settings ...
do-not-edit-topic-sources-list = # Generated by AOSC Topic Manager. DO NOT EDIT THIS FILE!\n
select-topics-dialog = Select topic(s) to enroll in testing updates, deselect to rollback to stable:
removing-topic = Removing topic: {$name}

# pkg
can-not-get-pkg-from-database = Can not get package: {$name} from database.
can-not-get-pkg-version-from-database = Can not get package: {$name} {$version} from database.
can-not-get-package-with-branch = Can not get package {$name} with {$branch} branch.
search-pkg-no-result = Could not find any packages for keyword: {$input}
debug-symbol-available = (debug symbols available)
full-match = full match
already-installed = {$name} {$version} os already installed
can-not-mark-reinstall = Pkg: {$name} {$version} cannot be marked for reinstallation as the specified version {$version} could not be found in any enabled repository.
mayble-dep-issue = {$name} can't marked installed! maybe dependency issue?
pkg-is-essential = Pkg {$name} is essential, so can not mark it as deleted

# pager
question-tips-with-x11 = Press [q] to end review, [Ctrl-c] to abort, [PgUp/Dn], arrow keys, or mouse wheel to scroll.
normal-tips-with-x11 = "Press [q] or [Ctrl-c] to exit, [PgUp/Dn], arrow keys, or mouse wheel to scroll.
question-tips = Press [q] to end review, [Ctrl-c] to abort, [PgUp/Dn] or arrow keys to scroll.
normal-tips = Press [q] or [Ctrl-c] to exit, [PgUp/Dn] or arrow keys to scroll.

# oma
no-need-to-do-anything = No need to do anything.
retry-apt = apt has retrn non-zero code, retrying {$count} times
system-has-broken-dep = Your system has broken dependencie
system-has-broken-dep-due-to = Try to use {$cmd} to fix broken dependencies, If this does not work, please contact upstream: https://github.com/aosc-dev/aosc-os-abbs
additional-version = There is {$len} additional version. Please use the '-a' switch to see it
could-not-find-pkg-from-keyword = Could not find any package for keyword {$c}
broken-by = "Broken by"
pre-depended-by = "Pre-depended by"
successfully-download-to-path = Successfully downloaded {$len} packages to path: {$path}
no-need-to-remove = Package {$name} is not installed, so no need to remove.
packages-can-be-upgrade = {$len} package(s) can be upgraded
packages-can-be-removed = {$len} package(s) can be removed
run-oma-upgrade-tips = Run 'oma upgrade' to see it
comma = ,
full-comma = .
successfully-refresh-with-tips = Successfully refreshed package database. {$s}
successfully-refresh = Successfully refreshed package database. No update available.
no-candidate-ver = {$pkg} has no candidate version.
pkg-is-not-installed = {$pkg} is not installed!
dpkg-data-is-broken = dpkg data is broken!
already-hold = {$name} is already hold!
set-to-hold = {$name} is set to hold.
already-unhold =  {$name} is already unhold!
set-to-unhold = {$name} is set to unhold.
already-manual = {$name} is already manual installed status.
setting-manual = Setting {$name} to manual installed status ...
already-auto = {$name} is already auto installed status.
setting-auto = Setting {$name} to auto installed status ...
command-not-found-with-result = Command not found: {$kw}. This command may be found from the following package(s):
command-not-found = Command not found: {$kw}
clean-successfully = Clean successfully.
dpkg-get-selections-non-zero = dpkg --get-selections return non-zero code.
can-not-parse-line =  Can not parse line {$i}
dpkg-was-interrupted = dpkg was interrupted, running
dpkg-configure-a-non-zero = Running `dpkg --configure -a` return non-zero code:
verifying-the-interity-of-pkgs = Verifying the integrity of packages ...
automatic-mode-warn = Now you are using automatic mode, if this is not your intention, press Ctrl + C to stop the operation!!!!!
has-no-symbol-pkg = {$name} has no debug symbol package!
pkg-no-version = Can not get version of package: {$name}
removed-as-unneed-dep = removed as unneeded dependency
purge-file = purge configuration files
semicolon = ;
should-installed = BUG: package {$name} should installed, please report this to upstream: https://github.com/aosc-dev/oma

# main
user-aborted-op = User aborted the operation

# formatter
download-not-done = Download is not done, running apt download ...\nIf you are not install package from local sources/local packages, Please run debug mode and report to upstream: https://github.com/aosc-dev/oma
force-auto-mode = Now you are using FORCE automatic mode, if this is not your intention, press Ctrl + C to stop the operation!!!!!
dpkg-force-all-mode = Now you are using DPKG FORCE ALL mode, if this is not your intention, press Ctrl + C to stop the operation!!!!!
dep-does-not-exist = Dep: {$name} does not exist
count-pkg-has-desc = {$count} package(s) has
dep-error = Dependency Error
dep-error-desc-1 = Omakase has detected dependency errors(s) in your system and cannot proceed with
dep-error-desc-2 = your specified operation(s). This may be caused by missing or mismatched\npackages, or that you have specified a version of a package that is not
dep-error-desc-3 = compatible with your system.
contact-admin-tips = Please contact your system administrator or developer
how-to-abort = Press [q] or [Ctrl-c] to abort.
how-to-op-with-x = Press [PgUp/Dn], arrow keys, or use the mouse wheel to scroll.
end-review = Press [q] to end review
cc-to-abort = Press [Ctrl-c] to abort
how-to-op = Press [PgUp/Dn] or arrow keys to scroll.
total-download-size = Total download size:
change-storage-usage = Estimated change in storage usage:
pending-op = Pending Operations
review-msg-1 = Shown below is an overview of the pending changes Omakase will apply to your
review-msg-2 = system, please review them carefully.
removed = REMOVED
installed = installed
upgrade = upgrade
downgraded = downgraded
reinstall = reinstall
oma-may-1 = Omakase may {$a}, {$b}, {$c}, {$d}, or {$e} packages in order
oma-may-2 = to fulfill your requested changes.
install = install
remove = remove
upgrade = upgrade
downgrade = downgrade

# download
invalid-url = URLs is none or Invalid URL
invaild-filename = Invalid file name: {$name}
invaild-ver = Invalid version: {$ver}
checksum-mismatch-try-next-url = {$c} checksum mismatch, try next url to download this package ...
checksum-mismatch-retry = {$c} checksum mismatch, retry {$retry} times ...
can-not-get-source-next-url = Can not get source, why: {$e}, try next url to download this package ..."
checksum-mismatch = Can not download file: {$filename} to dir {$dir}, checksum mismatch.
maybe-mirror-syncing = Maybe mirror still sync progress?
can-not-download-file = Can not download file: {$filename}, why: {$e}
check-network-settings = Maybe check your network settings?
can-not-download-file-with-why =  Can not download file: {$filename} to dir {$dir}, why: {$e}
downloading-count-pkg = Downloading {$count} packages ...
progress = Progress:
success-download-pkg = Downloaded {$download_len} package.
no-need-to-fetch-anything = No need to Fetch anything.
can-not-get-filename = BUG: Can not get filename {$name}! please report this to upstream: https://github.com/aosc-dev/oma

# db
setting-path-does-not-exist = Dir::Cache::Archives path: {$path} does not exist! fallback to /var/cache/apt/archives"
invaild-url-with-err = Invalid URL: {$url}, why: {$e}
cant-parse-distro-repo-data = Can not parse distro-repository-data file: {$mirror}, why: {$e}, maybe you environment is broken?
distro-repo-data-invalid-url = Distro repository data file have invalid URL: {$url}, why: {$e}
host-str-err = Can not get host str from mirror map!
can-nnot-read-inrelease-file = Can not parse InRelease file: {$path}, why: {$e}
inrelease-date-empty = InRelease Date entry is empty!
inrelease-valid-until-empty = InRelease Valid-Until entry is empty!
can-not-parse-date = BUG: can not parse data field: {$date} to Rfc2822, Please report this error to upstream: https://github.com/aosc-dev/oma
can-not-parse-valid-until = BUG: can not parse valid_until field: {$valid_until} to Rfc2822, Please report this error to upstream: https://github.com/aosc-dev/oma
earlier-signature = The computer time is earlier than the signature time in InRelease.
expired-signature = InRelease signature file has expired
inrelease-sha256-empty = InRelease sha256 entry is empty!
inrelease-checksum-can-not-parse = Could not parse checksum entry: {$i}
inrelease-parse-unsupport-file-type = BUG: InRelease Parser unsupport file type, Please report this to upstream: https://github.com/aosc-dev/oma
can-not-parse-sources-list = Can not scan and parse source.list file, why: {$e}
unsupport-cdrom = Omakase does not support cdrom protocol in url: {$url}
unsupport-some-mirror = Omakase unsupport some mirror.
unsupport-sourceentry = Unsupport SourceEntry:
refreshing-repo-metadata = Refreshing local repository metadata ...
can-not-get-suite = Can not get suite: {$url}
not-found = Could not get InRelease in url: {$url}, Reason: 404 Not found.
contents = Contents
pkg_list = Package List
bincontents = BinContents
decompressing = Decompressing
unsupport-decompress-file = BUG: unsupport decompress file: {$name}, Please report this error to upstream: https://github.com/aosc-dev/oma

# contents
contents-does-not-exist = Contents database does not exist! Please use {$cmd} to refresh the contents.
contents-may-not-be-accurate = Contents file: {$file} has not been updated for a week, so the search results may not be accurate, please use 'oma refresh' to refresh the database."
execute-ripgrep-failed = Execute rg failed, why: {$e}
searching = Searching ...
parse-rg-result-failed = BUG: Parse rg result {$i} failed, why: {$e}, Please report to upstream: https://github.com/aosc-dev/oma
search-with-result-count = Searching, found {$count} results so far ...
can-not-parse-search
contents-entry-missing-path-list = BUG: Contents entry: {$entry} missing path last, Please report this to upstrem: https://github.com/aosc-dev/oma
rg-non-zero = ripgrep return not-zero code!
