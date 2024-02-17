local util = require('nekifoch.utils')
local func = require('nekifoch.command_func')

vim.api.nvim_create_user_command('Nekifoch', function(opts)
  if opts.fargs[1] == 'set_font' then
    func.set_font(opts)
  elseif opts.fargs[1] == 'list' then
    func.list()
  elseif opts.fargs[1] == 'check' then
    func.check()
  elseif opts.fargs[1] == 'set_size' then
    func.set_size(opts)
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
        return util.compareFontsWithKittyListFonts(
          util.getCachedInstalledFonts()
        )
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
