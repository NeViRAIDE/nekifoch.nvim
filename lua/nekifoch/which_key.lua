local wk = require('which-key')

wk.add({
  {'<leader>sf',  group = 'Font' },
  {'<leader>sfl' ,  '<cmd>Nekifoch list<cr>', desc = 'Fonts list' },
  {'<leader>sfc' ,  '<cmd>Nekifoch check<cr>', desc = 'Check current font settings' },
  {'<leader>sff', rhs = function() require('nekifoch.nui_set_font')() end, desc = 'Set font family' },
  {'<leader>sfs', rhs = function() require('nekifoch.nui_set_size')() end, desc = 'Set font size' },
})
