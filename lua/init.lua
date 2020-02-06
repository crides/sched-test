add_log("run file init.lua", "The logging process has started up")
add_log_type({
    name = "test",
    attrs = {
        finished = {},
        finish_time = {
            hidden = true,
            default = "now",
        },
    },
})

function add_log_interactive(name)
    if name then
        log_type = get_log_type(name)
        if not log_type then
            print("Invalid log type")
            return
        end
    end
    name, _err = readline("Name: ")
    if not name then return end
    desc, _err = readline("Description: ")
    if not desc then return end
    props = {}
    if log_type then
        for attr, attr_prop in pairs(log_type) do
            if attr_prop.hidden then
                default = attr_prop.default
                if type(default) == "function" then
                    props[attr] = default()
                else
                    props[attr] = default
                end
            else
                attr_content, _err = readline(attr .. ": ")
                if not attr_content then return end
                props[attr] = attr_content
            end
        end
    end
    add_log_with_props(name, desc, props)
end

repl()
