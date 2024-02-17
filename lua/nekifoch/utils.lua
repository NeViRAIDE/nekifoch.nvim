local M = {}

---@return table
M.get = function()
  local f = assert(io.open(require('nekifoch').config.kitty_conf_path, 'r'))
  local content = f:read('*all')
  f:close()

  local current_font_family, current_font_size
  for line in content:gmatch('[^\n]+') do
    if line:match('^font_family%s+[^%d]+') then
      current_font_family = line:match('^font_family%s+(.+)$')
    elseif line:match('^font_size%s+%d+') then
      current_font_size = line:match('^font_size%s+(%d+)')
    end
  end

  return { font = current_font_family, size = current_font_size }
end

M.listInstalledFonts = function()
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

M.compareFontsWithKittyListFonts = function(installedFonts)
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

  table.sort(compatibleFonts)

  return compatibleFonts
end

M.replace_font_family = function(...)
  local new_font_family = table.concat({ ... }, ' ')
  local f = assert(io.open(require('nekifoch').config.kitty_conf_path, 'r'))
  local content = f:read('*all')
  f:close()

  local modified_content =
    content:gsub('font_family.-\n', 'font_family ' .. new_font_family .. '\n')

  f = assert(io.open(require('nekifoch').config.kitty_conf_path, 'w'))
  f:write(modified_content)
  f:close()
end

---@param size integer
M.replace_font_size = function(size)
  local f = assert(io.open(require('nekifoch').config.kitty_conf_path, 'r'))
  local content = f:read('*all')
  f:close()

  local modified_content =
    content:gsub('font_size.-\n', 'font_size ' .. size .. '\n')

  f = assert(io.open(require('nekifoch').config.kitty_conf_path, 'w'))
  f:write(modified_content)
  f:close()
end

-- Cache for installed fonts
M.cachedInstalledFonts = nil

---Get list of installed fonts with caching mechanism
---@return table
M.getCachedInstalledFonts = function()
  if not M.cachedInstalledFonts then
    M.cachedInstalledFonts = M.listInstalledFonts()
  end
  return M.cachedInstalledFonts
end

return M
