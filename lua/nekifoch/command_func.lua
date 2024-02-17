local util = require('nekifoch.utils')

local M = {}

M.set_font = function(opts)
  local args = {}
  for i = 2, #opts.fargs do
    table.insert(args, opts.fargs[i])
  end
  util.replace_font_family(unpack(args))

  if vim.fn.systemlist('pidof kitty')[1] ~= '' then
    vim.cmd('silent !kill -USR1 $(pidof kitty)')
  end
end

M.set_size = function(opts)
  util.replace_font_size(opts.fargs[2])
  if vim.fn.systemlist('pidof kitty')[1] ~= '' then
    vim.cmd('silent !kill -USR1 $(pidof kitty)')
  end
end

M.list = function()
  local availableFonts =
    util.compareFontsWithKittyListFonts(util.getCachedInstalledFonts())
  print('Available fonts:')
  for _, font in ipairs(availableFonts) do
    print(' - ' .. font)
  end
end

M.check = function()
  local current_font = util.get()
  if current_font then
    vim.notify(
      current_font['font'] .. '\n\t' .. 'Font size: ' .. current_font['size'],
      2,
      { title = 'Current font' }
    )
  else
    vim.notify('Font family not found in configuration', 4, { title = 'Font' })
  end
end

return M
