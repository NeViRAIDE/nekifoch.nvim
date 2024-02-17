local wk = require('which-key')

wk.register({
  ['<leader>'] = {
    s = {
      name = 'Settings',
      f = {
        name = 'Font',
        l = { ':Nekifoch list<cr>', 'Fonts list' },
        c = { ':Nekifoch check<cr>', 'Check current font settings' },
        f = { ':Nekifoch set_font<cr>', 'Set font family' },
        s = { ':Nekifoch set_size<cr>', 'Set font size' },
      },
    },
  },
})
