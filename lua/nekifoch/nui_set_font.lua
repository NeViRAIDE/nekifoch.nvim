return function()
  local Menu = require('nui.menu')
  local event = require('nui.utils.autocmd').event
  local util = require('nekifoch.utils')

  local fontMenuItems = {}
  local compatibleFonts =
    util.compareFontsWithKittyListFonts(util.getCachedInstalledFonts())
  for _, font in ipairs(compatibleFonts) do
    table.insert(fontMenuItems, Menu.item(font))
  end

  local menu = Menu({
    position = '50%',
    size = {
      width = 40,
      height = 10,
    },
    buf_options = { filetype = 'nekifoch' },
    border = {
      padding = { 0, 1 },
      style = require('nekifoch').config.borders,
      text = {
        top = ' Set font family ',
        top_align = 'center',
      },
    },
    win_options = {
      winhighlight = 'NormalFloat:NormalFloat,FloatBorder:FloatBorder',
    },
  }, {
    lines = fontMenuItems,
    max_width = 20,
    keymap = {
      focus_next = { 'j', '<Down>', '<Tab>' },
      focus_prev = { 'k', '<Up>', '<S-Tab>' },
      close = { '<Esc>', '<C-c>' },
      submit = { '<CR>', '<Space>' },
    },
    -- on_close = function() print('Menu Closed!') end,
    on_submit = function(item)
      util.replace_font_family(item.text)
      if vim.fn.systemlist('pidof kitty')[1] ~= '' then
        vim.cmd('silent !kill -USR1 $(pidof kitty)')
      end
    end,
  })

  menu:on(event.BufLeave, function() menu:unmount() end)
  menu:map(
    'n',
    { '<Esc>', 'q', '<C-c>' },
    function() menu:unmount() end,
    { noremap = true, nowait = true }
  )
  menu:map(
    'i',
    { '<Esc>', '<C-q>', '<C-c>' },
    function() menu:unmount() end,
    { noremap = true, nowait = true }
  )
  -- mount the component
  menu:mount()
end
