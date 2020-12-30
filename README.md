# pulga [![CodeFactor](https://www.codefactor.io/repository/github/carmesim/pulga/badge)](https://www.codefactor.io/repository/github/carmesim/pulga) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/carmesim/pulga/BuildAndValgrind)

**Early work in progress**

`Pulga` aims to be a customizable, highly performant command-line system information tool.

![Sample](https://user-images.githubusercontent.com/36349314/103319031-1f002480-4a0f-11eb-96bd-e4bdaa16d853.png)

## Performance

Different than similar tools, such as [Neofetch](https://github.com/dylanaraps/neofetch) and [pfetch](https://github.com/dylanaraps/pfetch), which are written in scripting languages, Pulga is written in Rust, focusing on obtaining all of its data solely using the standard libraries of Rust and C, alongside system files. Pulga currently runs in under 10 milliseconds even on low-end hardware.

### Quick benchmark

On a Raspberry 3 Model B running Raspbian 10, we ran the following command:

```shell
hyperfine --warmup 5 "./pulga" "./neofetch"
```

Pulga ([as of this commit](https://github.com/carmesim/pulga/commit/b82da05bf886ae6e87131c63a89da94a3b19edd8)) had a mean runtime of 6.0 ms ± 2.1 ms.

Neofetch ([as of this commit](https://github.com/dylanaraps/neofetch/commit/6dd85d67fc0d4ede9248f2df31b2cd554cca6c2f)) had a mean runtime of 1.281 s ± 0.064 s.

Overall:
```
  './pulga' ran
  213.03 ± 76.66 times faster than './neofetch'
```

This is, of course, not a 'fair' match since Pulga does not yet offer feature-parity with Neofetch.

## Memory safety

Even though Pulga makes use of [Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) and FFI with the C standard library, Pulga does not do away with memory safety.

At every commit, the GitHub Actions workflow runs Pulga under [Valgrind](https://valgrind.org/). If Valgrind encounters any error, the build is considered to be a failure.

## To do

- [x] Display username and hostname
- [x] Display the number of logical CPU cores
- [x] Display the maximum CPU frequency
- [x] Display uptime
- [x] Display the default shell
- [x] Display the distro on `systemd`-based systems
- [x] Display the kernel version
- [x] Display current memory usage
- [x] Display total memory available
- [x] Display the current desktop environment
  * Currently implemented but lacks some DEs still
- [ ] Display the current window manager
- [ ] Display the terminal being used
- [ ] Add the ability to customize Pulga through a configuration file located in `~/.config/pulga.toml`.
- [ ] Display storage usage
- [ ] Display screen resolution
- [ ] Add logos for more Linux distributions
- [ ] Add command-line arguments
- [ ] Display the distro on non-`systemd`-based systems
- [ ] Display the default editor
- [x] Display CPU model
- [ ] Display GPU model

## Non-goals

Pulga currently focuses solely on Linux distros. Supporting other Unix-like OSes is possible in the future. Supporting Windows is a non-goal.
