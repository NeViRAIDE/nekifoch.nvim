local util = require('nekifoch.utils')

-- Cache for installed fonts
local cachedInstalledFonts = nil

---Get list of installed fonts with caching mechanism
---@return table
local function getCachedInstalledFonts()
  if not cachedInstalledFonts then
    cachedInstalledFonts = util.listInstalledFonts()
  end
  return cachedInstalledFonts
end

vim.api.nvim_create_user_command('Nekifoch', function(opts)
  if opts.fargs[1] == 'set_font' then
    local args = {}
    for i = 2, #opts.fargs do
      table.insert(args, opts.fargs[i])
    end
    util.replace_font_family(unpack(args))

    if vim.fn.systemlist('pidof kitty')[1] ~= '' then
      vim.cmd('silent !kill -USR1 $(pidof kitty)')
    end
  elseif opts.fargs[1] == 'list' then
    local availableFonts =
      util.compareFontsWithKittyListFonts(getCachedInstalledFonts())
    print('Available fonts:')
    for _, font in ipairs(availableFonts) do
      print(' - ' .. font)
    end
  elseif opts.fargs[1] == 'check' then
    local current_font = util.get()
    if current_font then
      vim.notify(
        current_font['font'] .. '\n\t' .. 'Font size: ' .. current_font['size'],
        2,
        { title = 'Current font' }
      )
    else
      vim.notify(
        'Font family not found in configuration',
        4,
        { title = 'Font' }
      )
    end
  elseif opts.fargs[1] == 'set_size' then
    util.replace_font_size(opts.fargs[2])
    if vim.fn.systemlist('pidof kitty')[1] ~= '' then
      vim.cmd('silent !kill -USR1 $(pidof kitty)')
    end
  end
end, {
  nargs = '*',
  desc = 'Replace font family in Kitty configuration file',

  complete = function(findstart)
    if findstart == 1 then
      return vim.fn.col('.') - 1
    else
      local args = vim.fn.split(vim.fn.getcmdline(), ' ')
      local current_arg = args[2]

      if current_arg == 'set_font' then
        return util.compareFontsWithKittyListFonts(getCachedInstalledFonts())
      elseif
        current_arg == 'list'
        or current_arg == 'set_size'
        or current_arg == 'check'
      then
        return {}
      else
        return { 'check', 'set_font', 'set_size', 'list' }
      end
    end
  end,
})
