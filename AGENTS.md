# Release requirements

When the user asks to make a release, build and publish the complete package set
below by default. The user does not need to list the formats again. Use a smaller
set only when the user explicitly requests it.

Required packages for x86-64:

- Windows NSIS installer: `*-windows-x64-setup.exe`.
- Windows portable executable: `*-windows-x64-portable.exe`.
- Windows ZIP archive: `*-win-x64.zip`.
- Linux AppImage: `*-linux-x86_64.AppImage`.
- Debian/Ubuntu package: `*-linux-amd64.deb`.
- RPM package: `*-linux-x86_64.rpm`.
- Linux tar.gz archive: `*-linux-x64.tar.gz`.

Publish all seven packages together in one GitHub Release in `MX-Goliath/numi`.
Include `SHA256SUMS.txt` covering all seven packages and release notes describing
the changes and supported platforms.

AppImages must embed the standard `.upd_info` GitHub update channel and include
their generated `.AppImage.zsync` control file in the release and checksums.
This metadata is for external AppImage managers; do not add an in-app updater or
update settings interface unless the user explicitly asks for one.

Build every package from the release commit. Build the Windows native Rust module
on Windows and the Linux module on Linux. Run the engine tests, native bridge
tests, and packaged application smoke tests on both platforms before publishing.
The packaged startup checks must also pass with `ELECTRON_RUN_AS_NODE=1` inherited
from the environment. Do not publish an incomplete release or reuse stale binaries.

Use `.github/workflows/release.yml` for the builds and publication. Before starting
a new release, update the application version, lockfile, workflow tag, artifact
paths, and release notes to match the requested version. The existing workflow
contains values specific to `v1.0`; do not rerun it unchanged for a newer release.
