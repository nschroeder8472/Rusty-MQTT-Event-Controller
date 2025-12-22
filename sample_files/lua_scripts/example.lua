function handle_message(message)
    if tonumber(message) == 1 then
        publish_message("example/publish/topic", "message")
        return true
    end
    return false
end