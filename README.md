# Fast GUI password generator

## Make sure you installed libadwaita
## Linux
Fedora and derivatives:
```
sudo dnf install libadwaita-devel
```
Debian and derivatives:
```
sudo apt install libadwaita-1-dev
```
Arch and derivatives:
```
sudo pacman -S libadwaita
```
macOS
```
brew install libadwaita
```
Windows
If using gvsbuild

If you used gvsbuild to build GTK 4:
```
gvsbuild build libadwaita librsvg
```

## Goal

Simple password generator, that can be used anywhere, anytime, by using a keybind or executing in app launcher.

## Key concepts

- Small windowed ui, obstructing as little as possible;
- File-based configuration;
- Fast.

## Why?

Because I'm bored generating my password in terminal or doing it in web browser

## Progress

- [X] Random string generator;
- [ ] UI;
- [ ] Support for conf file;
- [ ] .desktop file;
- [ ] Installer.
