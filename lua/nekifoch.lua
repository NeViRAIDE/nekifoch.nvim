---@class FontReplaceConfig
---@field kitty_conf_path string

local M = {}

---@return string
local get = function()
  local f = assert(io.open(M.config.kitty_conf_path, 'r'))
  local content = f:read('*all')
  f:close()

  local current_font_family = content:match('[^%S]font_family(.-)\n')
  if current_font_family then
    current_font_family = current_font_family:gsub('^%s*(.-)%s*$', '%1') -- Use the utility function
  end
  return current_font_family
end

local replace_font_family = function(new_font_family)
  local f = assert(io.open(require('nekifoch').config.kitty_conf_path, 'r'))
  local content = f:read('*all')
  f:close()

  local modified_content =
    content:gsub('font_family.-\n', 'font_family ' .. new_font_family .. '\n')

  f = assert(io.open(require('nekifoch').config.kitty_conf_path, 'w'))
  f:write(modified_content)
  f:close()
end

local function listInstalledFonts()
  local cmd = "fc-list : family 2>/dev/null | awk -F ',' '{print $1}'"
  local handle = io.popen(cmd)
  if not handle then
    return {} -- Return an empty list if the command couldn't be executed
  end
  local result = handle:read('*a')
  handle:close()

  local installedFonts = {}
  local hash = {}

  for font in result:gmatch('[^\r\n]+') do
    if font then
      if not hash[font] then
        installedFonts[#installedFonts + 1] = font
        hash[font] = true
      end
    end
  end

  return installedFonts
end

local function compareFontsWithKittyListFonts(installedFonts)
  local handle = io.popen('kitty +list-fonts 2>/dev/null') -- Redirect stderr to /dev/null
  if not handle then
    return {} -- Return an empty list if the command couldn't be executed
  end
  local result = handle:read('*a')
  handle:close()

  local kittyFonts = {}

  for font in result:gmatch('[^\r\n]+') do
    table.insert(kittyFonts, font)
  end

  local compatibleFonts = {}

  for _, font in ipairs(installedFonts) do
    local found = false
    for _, kittyFont in ipairs(kittyFonts) do
      if kittyFont:match(font) then
        found = true
        break
      end
    end
    if found then table.insert(compatibleFonts, font) end
  end

  return compatibleFonts
end

-- Usage
local availableFonts = compareFontsWithKittyListFonts(listInstalledFonts())

vim.notify('Available fonts:' .. vim.inspect(availableFonts))

---@type FontReplaceConfig
M.config = {
  kitty_conf_path = vim.env.HOME .. '/.config/kitty/kitty.conf',
}

function M.setup(config)
  M.config = vim.tbl_deep_extend('force', M.config, config or {})

  vim.api.nvim_create_user_command('Nekifoch', function(opts)
    if opts.fargs[1] == 'write' then
      replace_font_family(opts.fargs[2])
    elseif opts.fargs[1] == 'list' then
      print(vim.inspect(listInstalledFonts()))
    elseif opts.fargs[1] == 'check' then
      ---@type string
      local current_font_family = get()
      if current_font_family then
        vim.notify(current_font_family, 2, { title = 'Font' })
      else
        vim.notify(
          'Font family not found in configuration',
          4,
          { title = 'Font' }
        )
      end
    end
  end, {
    nargs = '*',
    desc = 'Replace font family in Kitty configuration file',
    complete = function()
      local commands = { 'check', 'write', 'list' }
      return commands
    end,
  })
end

return M
