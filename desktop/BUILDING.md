# Building Numi desktop

The desktop sources were recovered from the 0.1.15 Windows release. The calculator engine is Rust and the interface is React/Vite inside Electron.

Requirements: Node.js 22.12+, npm, Rust/Cargo, and a C compiler.

```sh
npm ci
npm run build
npm test
npm run appimage
```

`npm run appimage` writes `artifacts/Numi-1.0.0-linux-x86_64.AppImage`. Run it with `chmod +x artifacts/Numi-1.0.0-linux-x86_64.AppImage && ./artifacts/Numi-1.0.0-linux-x86_64.AppImage`.

Packaged builds disable Electron's RunAsNode fuse so they open normally even when a terminal or file manager passes down `ELECTRON_RUN_AS_NODE=1`.

On Windows, run `npm run windows` from a Windows checkout with Rust and the MSVC build tools. That builds the native `.node` module for Windows and creates an NSIS installer, a portable executable, and a ZIP archive. The source does not depend on the release archive or the original update servers.

Run `npm run linux` to build AppImage, DEB, RPM, and tar.gz packages for Linux x64. DEB and RPM packaging needs the distribution packaging tools.

The GitHub Actions workflow `Build release` builds Linux packages on Ubuntu 22.04 and Windows packages on Windows Server 2022. It runs the engine tests and checks that the packaged interface and calculator work even with `ELECTRON_RUN_AS_NODE=1`. The downloadable build artifacts contain the files for a GitHub Release.

The calculator works offline for arithmetic, dates, functions, and unit conversions. Currency rates refresh when the network is available and use cached values otherwise.

Wave quantities are available as `Hz`, SI-scaled frequencies such as `kHz` and `MHz`, `period`, `wavelength`, `angular frequency` (`ω`), and `wave number` (`k`). Ordinary units can be used as targets too: `1 Hz in s`, `1 Hz in m`, or `1 kHz in km`. Frequency and period are reciprocal; wavelength conversions assume propagation in vacuum (`c = 299792458 m/s`), and angular frequency and wave number use `ω = 2πf` and `k = 2π/λ`.
