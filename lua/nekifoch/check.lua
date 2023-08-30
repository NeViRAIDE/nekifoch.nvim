local util = require('nekifoch.utils')

local M = {}

M.check = function(font)
  local has = util.check_font(font)
  if has then
    vim.notify(font .. ' is already installed', 2, { title = 'All is OK' })
  else
    vim.notify(font .. ' is NOT installed', 3, { title = 'Problem' })
  end
end

return M
