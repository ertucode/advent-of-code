local M = {}

M.readAll = function(file)
	local f = assert(io.open(file, "rb"))
	local content = f:read("*all")
	f:close()
	return content
end

M.string_lines = function(s)
	if s:sub(-1) ~= "\n" then
		s = s .. "\n"
	end
	return s:gmatch("(.-)\n")
end

M.string_split = function(s, sep)
	local fields = {}

	local sep = sep or " "
	local pattern = string.format("([^%s]+)", sep)
	string.gsub(s, pattern, function(c)
		fields[#fields + 1] = c
	end)

	return fields
end

M.array_map = function(arr, cb)
	local ret = {}
	for index, value in ipairs(arr) do
		table.insert(arr, cb(value, index))
	end
	return ret
end

M.table_print = function(t)
	if type(t) == "table" then
		local s = "{ "
		for k, v in pairs(t) do
			if type(k) ~= "number" then
				k = '"' .. k .. '"'
			end
			s = s .. "[" .. k .. "] = " .. M.table_print(v) .. ","
		end
		return s .. "} "
	else
		return tostring(t)
	end
end

return M
