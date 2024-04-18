-- TODO: remake all
-- FIX: autocompletion after entered font and second argument

local util = require('nekifoch.utils')
local func = require('nekifoch.command_func')

local M = {}

function M.setup()
  vim.api.nvim_create_user_command('Nekifoch', function(opts)
    local cmd = vim.trim(opts.args):match('^(%S+)') -- Извлекаем первое слово как команду
    if M.commands[cmd] then
      M.commands[cmd](opts)
    else
      print('Command not found')
    end
  end, {
    nargs = '*',
    desc = 'Replace font family in Kitty configuration file',
    complete = M.complete,
  })
end

M.commands = {
  list = function() func.list() end,
  check = function() func.check() end,
  set_font = function(opts) func.set_font(opts) end,
  set_size = function(opts) func.set_size(opts) end,
}

function M.complete(findstart)
  if findstart == 1 then
    return vim.fn.col('.') - 1
  else
    local args = vim.fn.split(vim.fn.getcmdline(), ' ')
    local second_arg = args[2]
    if second_arg == 'set_font' then
      return M.handleSetFontComplete(args)
    elseif
      second_arg == 'list'
      or second_arg == 'set_size'
      or second_arg == 'check'
    then
      return {}
    else
      return { 'check', 'set_font', 'set_size', 'list' }
    end
  end
end

function M.handleSetFontComplete(args)
  if args[3] then
    local lineUntilCursor = vim.fn.getcmdline():sub(1, vim.fn.col('.') - 1)
    if lineUntilCursor:sub(-1) == ' ' then return {} end
    local partialFontName = table.concat(args, ' ', 3)
    local formattedFonts, _ =
      util.compareFontsWithKittyListFonts(util.getCachedInstalledFonts())
    if formattedFonts then
      local keys = vim.tbl_keys(formattedFonts)
      local filteredKeys = {}
      for _, key in ipairs(keys) do
        if key:lower():find(vim.pesc(partialFontName:lower()), 1, true) then
          table.insert(filteredKeys, key)
        end
      end
      table.sort(filteredKeys)
      return filteredKeys
    else
      return {}
    end
  else
    local formattedFonts, _ =
      util.compareFontsWithKittyListFonts(util.getCachedInstalledFonts())
    if formattedFonts then
      local keys = vim.tbl_keys(formattedFonts)
      table.sort(keys)
      return keys
    else
      return {}
    end
  end
end

return M
