local M = {}

function M.replace_word(old, new)
  local neviraide_conf = vim.fn.stdpath('config') .. '/lua/' .. 'NEVIRAIDE.lua'
  local file = io.open(neviraide_conf, 'r')
  local added_pattern = string.gsub(old, '-', '%%-') -- add % before - if exists
  if file ~= nil then
    local new_content = file:read('*all'):gsub(added_pattern, new)
    --
    file:close()
    --
    file = io.open(neviraide_conf, 'w')
    if file ~= nil then
      file:write(new_content)
      file:close()
    end
  end
end

---@param font string
---@return boolean
M.check_font = function(font)
  -- if font == nil then
  --   require('nekifoch.errors').no_font_arg()
  --   return nil
  -- end
  local has_font = vim.fn.system('fc-list | grep -i ' .. font)
  if has_font ~= '' then return true end
  return false
end

return M
