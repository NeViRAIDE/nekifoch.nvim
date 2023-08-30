local M = {}

M.low_args = function(opts)
  if #opts.fargs < 2 then
    vim.notify('You have to pass an arguments', 3, { title = 'Read docs' })
  end
end

M.no_font_arg = function()
  vim.notify(
    'You have to pass a font you wish to check',
    3,
    { title = 'Read docs' }
  )
end

return M
