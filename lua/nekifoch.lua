---@class FontReplaceConfig
---@field kitty_conf_path string

-- TODO: add reading from NEVIRAIDE config and config path to NEVIRAIDEfile
local M = {}

---@type FontReplaceConfig
M.config = {
  kitty_conf_path = vim.env.HOME .. '/.config/kitty/kitty.conf',
}

---Setup font configuration
---@param config table
function M.setup(config)
  M.config = vim.tbl_deep_extend('force', M.config, config or {})
  require('nekifoch.command')
end

return M
