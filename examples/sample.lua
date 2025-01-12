local red = "\27[31m"
local grn = "\27[32m"
local yel = "\27[33m"
local blu = "\27[34m"
local mag = "\27[35m"
local wht = "\27[37m"
local bold = "\27[1m"
local res = "\27[0m"

local turtle = {
    grn .. "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⣀⣀⣀⠀⠀⠀⠀" .. res,
    grn .. "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠊⠁⠀⠀⠀⠀⠈⠙⢦⡀⠀" .. res,
    grn .. "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡜⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢳⠀" .. res,
    grn .. "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⡇" .. res,
    grn .. "⠀⠀⠀⠀⠀⠀⠀⣀⠤⢄⣀⠀⠀⠀⡇⠀⠀⠀⠀⢰⣶⠄⠀⠀⠀⠀⡇" .. res,
    grn .. "⠀⠀⠀⠀⠀⡴⡋⠀⠀⠀⡨⠓⣄⠀⢳⠀⠀⠀⠀⠀⠉⠀⠀⠀⢀⡼⠀" .. res,
    grn .. "⠀⠀⠀⢀⡞⠀⢸⠓⠒⢺⡀⠀⠈⢣⠈⡇⠀⠀⠀⠀⠀⢠⡤⠴⠋⠀⠀" .. res,
    grn .. "⠀⠀⠀⡼⠒⠒⢏⠀⠀⠀⠙⣦⠖⠉⢧⡿⠀⠈⠙⡖⠚⠉⠀⠀⠀⠀⠀" .. res,
    grn .. "⠀⠀⡖⢧⡀⠀⠈⣦⡤⠤⠊⡏⣀⡴⠊⡹⠀⣠⠞⠀⠀⠀⠀⠀⠀⠀⠀" .. res,
    grn .. "⢶⡞⡟⠦⣌⡓⠾⠥⠤⠴⠒⠋⣁⠴⢊⣤⠞⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀" .. res,
    grn .. "⠀⠀⡇⠀⠀⢉⣙⣒⣒⣒⣒⣉⠁⠀⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀" .. res,
    grn .. "⠀⠀⠙⠒⠒⠚      ⠓⠚⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀" .. res,
}

local header = bold .. grn .. yafetch.user() .. res .. bold .. "@" .. bold .. red .. yafetch.hostname() .. res

local info = {
    res .. res,
    res .. res,
    header,
    res .. res,
    blu .. "distro       " .. res .. bold .. wht .. yafetch.os() .. res,
    blu .. "cpu          " .. res .. bold .. wht .. yafetch.cpu() .. res,
    blu .. "memory       " .. res .. bold .. wht .. yafetch.mem_used() .. " / " .. yafetch.mem_total() .. res,
    blu .. "disk         " .. res .. bold .. wht .. yafetch.disk_free("/") .. " / " .. yafetch.disk_total("/") .. res,
    blu .. "uptime       " .. res .. bold .. wht .. yafetch.uptime() .. res,
    blu .. "date/time    " .. res .. bold .. wht .. yafetch.current_datetime() .. res,
}

local max_lines = math.max(#turtle, #info)

for i = 1, max_lines do
    local turtle_line = turtle[i] or " "
    local info_line = info[i] or ""
    print(turtle_line .. "      " .. info_line)
end
