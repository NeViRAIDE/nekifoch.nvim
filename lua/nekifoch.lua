vim.api.nvim_create_user_command('Nekifoch', function(opts)
  if opts.fargs[1] == 'check' then
    require('nekifoch.check').check(opts.fargs[2])
  elseif opts.fargs[1] == 'install' then
    require('nekifoch.install').install(opts.fargs[2])
  elseif opts.fargs[1] == 'write' then
    require('nekifoch.write').write_to_kitty(opts.fargs[2])
  end
end, {
  nargs = '*',
  complete = function() return { 'check', 'install', 'write' } end,
})
