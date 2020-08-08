# Release note

| Date       | Version | Change                                                                                                                                                                    |
| ---------- | ------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 2020/08/08 | v2.0.6  | Add view subcommands(Same movement as -v option).                                                                                                                         |
| 2020/08/08 | v2.0.5  | Remove -f option.<br>Add show(rustup show command wrapper) and formatter(clippy and rustfmt install) subcommands.                                                         |
| 2020/03/29 | v2.0.4  | Add -i command. Add error pattern: RLS or Clippy build failed 7 days.                                                                                                     |
| 2020/02/20 | v2.0.3  | When clippy was successfully built, it assumed that rls was also successfully built.<br>but because the assumption was wrong, it was modified to check both build status. |
| 2019/12/04 | v2.0.2  | Always find the date.                                                                                                                                                     |
| 2019/11/23 | v2.0.1  | use reqwest -> ureq                                                                                                                                                       |
| 2019/11/09 | v2.0.0  | Fixed: Nightly rust now requires clippy, so check clippy build status.<br>Change: Removed latest.txt.<br>Other: Removed some examples.                                    |
| 2019/09/30 | v1.0.30 | Fix bug. Can't update stable and beta.                                                                                                                                    |
| 2019/09/02 | v1.0.29 | Update minimum guarantee(2019-05-22->2019-09-02) and quarity up.                                                                                                          |
| 2019/05/23 | v1.0.28 | Add 'u' option(`rustup uninstall` wrapper) and change env!, added dirs library.                                                                                           |
| 2019/05/21 | v1.0.27 | Add description of default-toolchain option.                                                                                                                              |
| 2019/05/20 | v1.0.26 | Improve help.                                                                                                                                                             |
| 2019/05/19 | v1.0.25 | Add `rustup default` wrapper.                                                                                                                                             |
| 2019/05/19 | v1.0.24 | Fix minor bug.                                                                                                                                                            |
| 2019/05/12 | v1.0.23 | Add `rustup component add` wrapper.                                                                                                                                       |
| 2019/04/20 | v1.0.22 | Follow change "eight to seven".                                                                                                                                           |
| 2019/04/12 | v1.0.21 | Accept "Last available".                                                                                                                                                  |
| 2019/03/29 | v1.0.20 | Display command only when necessary.                                                                                                                                      |
| 2019/03/26 | v1.0.19 | Bug fix: Can't install Nightly Rust.                                                                                                                                      |
| 2019/03/25 | v1.0.18 | Bug fix: Can't get the appropriate version.                                                                                                                               |
| 2019/03/24 | v1.0.17 | Add view option.                                                                                                                                                          |
| 2019/03/16 | v1.0.16 | Unnecessary data(tests and examples dir) was removed from inclusion.                                                                                                      |
| 2019/03/16 | v1.0.15 | Test enhancement.                                                                                                                                                         |
| 2019/03/14 | v1.0.14 | 1. Add CI setting files and Travis CI build passing marker.<br>2. Add test and example files.<br>3. Coding according to clippy.                                           |
| 2019/03/10 | v1.0.13 | Add more env macro.                                                                                                                                                       |
| 2019/03/09 | v1.0.12 | Version acquisition by env!("CARGO_PKG_VERSION").                                                                                                                         |
| 2019/03/09 | v1.0.11 | The repository "rustup-components-history" was incorporated into Rust's official Repository, so the URL was changed.                                                      |
| 2019/03/07 | v1.0.10 | Add installation Instructions.                                                                                                                                            |
| 2019/03/06 | v1.0.9  | README.md: There was an error in the description about dnf command.                                                                                                       |
| 2019/03/06 | v1.0.8  | Add description for installation with wsl in README.md.                                                                                                                   |
| 2019/03/06 | v1.0.7  | Fixed bug of v1.0.3 was not fixed.                                                                                                                                        |
| 2019/03/05 | v1.0.6  | README.md: fix tables.                                                                                                                                                    |
| 2019/03/05 | v1.0.5  | I set the installation location of latest.txt in the cargo-rls-install source folder in .cargo.                                                                           |
| 2019/03/05 | v1.0.4  | Create latest.txt in .cargo.                                                                                                                                              |
| 2019/03/05 | v1.0.3  | Fix fatal bug. Parse error always occurs in initial operation.                                                                                                            |
| 2019/03/04 | v1.0.2  | Minor change.                                                                                                                                                             |
| 2019/03/04 | v1.0.1  | Minor change.                                                                                                                                                             |
| 2019/03/04 | v1.0.0  | First release.                                                                                                                                                            |
