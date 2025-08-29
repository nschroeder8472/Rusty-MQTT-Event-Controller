function myFunction(message)
    if tonumber(message) == 3 then
        publish_message("test_on", "true")
        return true
    end
    return false
end