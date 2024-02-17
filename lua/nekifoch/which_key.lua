local wk = require('which-key')

wk.register({
  ['<leader>sf'] = { name = 'Font' },
  ['<leader>sfl'] = { ':Nekifoch list<cr>', 'Fonts list' },
  ['<leader>sfc'] = { ':Nekifoch check<cr>', 'Check current font settings' },
  ['<leader>sff'] = { ':Nekifoch set_font<cr>', 'Set font family' },
  ['<leader>sfs'] = {
    function() require('nekifoch.nui_set_size')() end,
    'Set font size',
  },
})
