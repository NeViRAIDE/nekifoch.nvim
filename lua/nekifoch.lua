---@class FontReplaceConfig
---@field kitty_conf_path string
---@field enable_extra_functionality boolean

-- TODO: add reading from NEVIRAIDE config and config path to NEVIRAIDEfile
local M = {}

---@type FontReplaceConfig
M.config = {
  kitty_conf_path = vim.env.HOME .. '/.config/kitty/kitty.conf',
  enable_extra_functionality = false, -- Default value: extra functionality is disabled
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
    M.config.enable_extra_functionality
    and is_plugin_installed('which-key')
    and is_plugin_installed('plenary.nvim')
  then
    -- Add your additional functionality here
    print(
      "Plugin 'which-key' and 'plenary.nvim' are installed and extra functionality is enabled."
    )
    -- Example: require('your_module').your_function()
  elseif M.config.enable_extra_functionality then
    print(
      "Extra functionality is enabled but plugin 'which-key' and/or 'plenary.nvim' are not installed. Additional functionality will not be available."
    )
  else
    print('Extra functionality is disabled.')
  end
end

return M
