# Building Numi desktop

The desktop sources were recovered from the 0.1.15 Windows release. The calculator engine is Rust and the interface is React/Vite inside Electron.

Requirements: Node.js 22+, npm, Rust/Cargo, and a C compiler.

```sh
npm install
npm test
npm run appimage
```

`npm run appimage` writes `artifacts/Numi-0.1.15.AppImage`. Run it with `chmod +x artifacts/Numi-0.1.15.AppImage && ./artifacts/Numi-0.1.15.AppImage`.

On Windows, run `npm run windows` from a Windows checkout with Rust and the MSVC build tools. That builds the native `.node` module for Windows and creates the NSIS installer. Linux can prepare the unpacked Windows directory, but cannot sign or build the NSIS installer without Wine. The source does not depend on the release archive or the original update servers.

The calculator works offline for arithmetic, dates, functions, and unit conversions. Currency rates refresh when the network is available and use cached values otherwise.

Wave quantities are available as `Hz`, SI-scaled frequencies such as `kHz` and `MHz`, `period`, `wavelength`, `angular frequency` (`ω`), and `wave number` (`k`). Ordinary units can be used as targets too: `1 Hz in s`, `1 Hz in m`, or `1 kHz in km`. Frequency and period are reciprocal; wavelength conversions assume propagation in vacuum (`c = 299792458 m/s`), and angular frequency and wave number use `ω = 2πf` and `k = 2π/λ`.
