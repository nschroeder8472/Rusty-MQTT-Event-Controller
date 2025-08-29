function myFunction(message)
    if tonumber(message) == 3 then
        publish_message("zwave/Master_Bedroom/MasterRoomLamp/37/0/targetValue/set", "true")
        return true
    end
    return false
end