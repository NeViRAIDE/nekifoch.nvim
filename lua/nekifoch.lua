---@class FontReplaceConfig
---@field kitty_conf_path string
---@field which_key WhichKeyNekifoch

---@class WhichKeyNekifoch
---@field enable boolean

-- TODO: add reading from NEVIRAIDE config and config path to NEVIRAIDEfile
local M = {}

---@type FontReplaceConfig
M.config = {
  kitty_conf_path = vim.env.HOME .. '/.config/kitty/kitty.conf',
  which_key = {
    enable = false,
  }, -- Default value: extra functionality is disabled
}

--- Check if a plugin is installed
---@param name string Name of the plugin to check
---@return boolean Whether the plugin is installed
local function is_plugin_installed(name)
  local ok, _ = pcall(require, name) -- Try to require the plugin
  return ok
end

--- Setup font configuration
---@param config table
function M.setup(config)
  M.config = vim.tbl_deep_extend('force', M.config, config or {})
  require('nekifoch.command')

  -- Check if which-key and plenary.nvim are installed
  if
    M.config.which_key.enable
    and is_plugin_installed('which-key')
    and is_plugin_installed('plenary')
  then
    require('nekifoch.which_key')
  elseif M.config.which_key.enable then
    print(
      "WhichKey functionality is enabled but plugin 'which-key' and/or 'plenary.nvim' are not installed. Additional functionality will not be available."
    )
  else
    print('WhichKey functionality is disabled.')
  end
end

return M
