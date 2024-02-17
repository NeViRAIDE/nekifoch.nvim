return function()
  local Input = require('nui.input')
  local event = require('nui.utils.autocmd').event

  local input = Input({
    position = { row = '50%', col = '50%' },
    size = { width = 20 },
    border = {
      style = 'rounded',
      padding = { 0, 1 },
      text = {
        top = ' Set font size ',
        top_align = 'center',
      },
    },
    win_options = {
      winhighlight = 'NormalFloat:NormalFloat,FloatBorder:FloatBorder',
    },
    relative = 'editor',
    buf_options = {
      filetype = 'create',
    },
  }, {
    prompt = '',
    default_value = '',
    on_submit = function(value)
      require('nekifoch.utils').replace_font_size(value)
      if vim.fn.systemlist('pidof kitty')[1] ~= '' then
        vim.cmd('silent !kill -USR1 $(pidof kitty)')
      end
    end,
  })
  input:on(event.BufLeave, function() input:unmount() end)
  input:map(
    'n',
    { '<Esc>', 'q', '<C-c>' },
    function() input:unmount() end,
    { noremap = true, nowait = true }
  )
  input:map(
    'i',
    { '<Esc>', '<C-q>', '<C-c>' },
    function() input:unmount() end,
    { noremap = true, nowait = true }
  )
  input:mount()
end
