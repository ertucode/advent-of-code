package.path = package.path .. ";../lib/init.lua"
local lua_lib = require("lib")

local input = lua_lib.readAll("./day1.input")

local elves = lua_lib.string_split(input, "/n")

print(lua_lib.table_print(elves))
print(#elves)
