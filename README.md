<div align="center">

# NeKiFoCh
*Neovim Kitty Font Changer*

[![Release](https://github.com/NeViRAIDE/nekifoch.nvim/actions/workflows/release.yml/badge.svg)](https://github.com/NeViRAIDE/nekifoch.nvim/actions/workflows/ci.yml)
![License](https://img.shields.io/github/license/NeViRAIDE/nekifoch.nvim)
![Neovim version](https://img.shields.io/badge/Neovim-0.10-57A143?logo=neovim)

</div>

<!--toc:start-->
- [NeKiFoCh](#nekifoch)
  - [Installation](#installation)
  - [Default config values](#default-config-values)
  - [Usage](#usage)
      - [Examples:](#examples)
  - [Configuration](#configuration)
  - [Credits](#credits)
  - [License](#license)
<!--toc:end-->

---

<div align="center">

**Neovim plugin for managing Kitty terminal font settings.**

</div>

---

https://github.com/RAprogramm/nekifoch/assets/70325462/04a0d7e7-a42e-4588-a926-0945adacb3f0

<div align="center">

## Installation

</div>

Install Nekifoch using your favorite plugin manager. For example, with [lazy.nvim](https://github.com/folke/lazy.nvim):

```lua
{
    'NeViRAIDE/nekifoch.nvim',
    build = 'make',
    cmd = 'Nekifoch',
    opts = {},
}
```

<div align=center id='defaults'>

## Default config values

</div>

```lua
{
    kitty_conf_path = '~/.config/kitty/kitty.conf',
    borders = 'none', --available values are: 'rounded', 'single', 'double', 'shadow', 'solid', 'none'
}
```

<div align=center>

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

- Check the current font family and size in float window:
```vim
:Nekifoch float_check
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

- List available fonts compatible with Kitty in float window:
```vim
:Nekifoch float_list
```

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
    build = 'make',
    cmd = 'Nekifoch', -- to add lazy loading
    opts = {
        kitty_conf_path = vim.fn.expand('~/.config/kitty/kitty.conf'), -- your kitty config path
    }
}
```

Replace '~/.config/kitty/kitty.conf' with the actual path to your Kitty terminal configuration.

<div align="center">

## Credits

Developed by [RAprogramm](https://github.com/RAprogramm). Contributions are welcome.

</div>

<div align="center">

## License

</div>

[APACHE 2.0](https://github.com/NeViRAIDE/nekifoch.nvim/blob/main/LICENSE)

For in-depth details and usage instructions, refer to the documentation.

---

Enhance your Kitty terminal experience with Nekifoch

