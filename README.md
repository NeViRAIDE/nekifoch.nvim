<h1 align="center" id='top'>NeKiFoCh</h1>
<h3 align="center">Neovim Kitty Font Changer</h3>

<div align="center">

[![Release](https://github.com/NeViRAIDE/nekifoch.nvim/actions/workflows/release.yml/badge.svg)](https://github.com/NeViRAIDE/nekifoch.nvim/actions/workflows/ci.yml)
![License](https://img.shields.io/github/license/NeViRAIDE/nekifoch.nvim)
![Neovim version](https://img.shields.io/badge/Neovim-0.10-57A143?logo=neovim)

</div>

<hr>

<p align="center">
  <strong>Neovim plugin for managing Kitty terminal font settings.</strong>
</p>

<p align="center">
  <a href="#installation">Installation</a> •
  <a href="#defaults">Defaults</a> •
  <a href="#usage">Usage</a> •
  <a href="#configuration">Configuration</a> •
  <a href="#credits">Credits</a> •
  <a href="#license">License</a> •
  <a href="#donations">Donations</a>
</p>

---

https://github.com/RAprogramm/nekifoch/assets/70325462/04a0d7e7-a42e-4588-a926-0945adacb3f0

<div align="center">

## Installation

</div>

Install Nekifoch using your favorite plugin manager. For example, with [lazy.nvim](https://github.com/folke/lazy.nvim):

```lua
{
    'NeViRAIDE/nekifoch.nvim',
    build = 'chmod +x ./install.sh && ./install.sh',
    cmd = 'Nekifoch',
    opts = {},
}
```

<br>
<p align="right">
  <a href="#top">. . . go top</a>
</p>

<h2 align=center id='defaults'>Default config values</h2>

```lua
{
    kitty_conf_path = '~/.config/kitty/kitty.conf',
    borders = 'none', --available values are: 'rounded', 'single', 'double', 'shadow', 'solid', 'none'
}
```
<br>
<p align="right">
  <a href="#top">. . . go top</a>
</p>

<div align="center">

## Usage

</div>

Nekifoch provides the `:Nekifoch` command with the following syntax:

```vim
:Nekifoch [action] [font_family/font_size]
```

- `[action]` can be one of:
  - `check`,
  - `set_font`,
  - `set_size`,
  - `list`.
- `[font_family/font_size]`: New font family/size for the `set_font`/`set_size` action.

#### Examples:

- Open NeKiFoCh main menu:

```vim
:Nekifoch
```

- Check the current font family and size:

```vim
:Nekifoch check
```

- Replace the font family with "DejaVu Sans Mono":

```vim
:Nekifoch set_font DejaVuSansMono
```

- Replace the font size with "14":

```vim
:Nekifoch set_size 14
```

- List available fonts compatible with Kitty:

```vim
:Nekifoch list
```

<br>
<p align="right">
  <a href="#top">. . . go top</a>
</p>

<div align="center">

## Configuration

</div>

Configure Nekifoch using the FontReplaceConfig dictionary:

Nekifoch can be configured by adding a Lua configuration to your Neovim configuration file (init.lua).

Here's an example configuration using Lua:

```lua
require('nekifoch').setup({
  kitty_conf_path = vim.fn.expand('~/.config/kitty/kitty.conf')
})
```

or

```lua
{
    'NeViRAIDE/nekifoch.nvim',
    build = 'chmod +x ./install.sh && ./install.sh',
    cmd = 'Nekifoch', -- to add lazy loading
    opts = {
        kitty_conf_path = vim.fn.expand('~/.config/kitty/kitty.conf'), -- your kitty config path
    }
}
```

Replace '~/.config/kitty/kitty.conf' with the actual path to your Kitty terminal configuration.
<br>
<p align="right">
  <a href="#top">. . . go top</a>
</p>




<div align="center">

## Credits

Developed by [RAprogramm](https://github.com/RAprogramm). Contributions are welcome.

</div>
<br>
<p align="right">
  <a href="#top">. . . go top</a>
</p>

<div align="center">

## License

</div>

[APACHE 2.0](https://github.com/NeViRAIDE/nekifoch.nvim/blob/main/LICENSE)

For in-depth details and usage instructions, refer to the documentation.

<hr>

Enhance your Kitty terminal experience with Nekifoch

<br>
<p align="right">
  <a href="#top">. . . go top</a>
</p>


<div align="center">

## Donations

</div>

If you find this plugin helpful and would like to support its development, you can buy me a coffee through the following platforms:


<div align="center">

[![ko-fi](https://www.ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/rozanov)

[![buymeacoffee](https://img.buymeacoffee.com/button-api/?username=YOUR_BMC_USERNAME&button_colour=FFDD00&font_colour=000000&font_family=Cookie&outline_colour=000000)](https://www.buymeacoffee.com/raprogramm)

Your support is greatly appreciated!

</div>

<br>
<p align="right">
  <a href="#top">. . . go top</a>
</p>
