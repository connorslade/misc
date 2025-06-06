FPS = nil
LAST_SLIDE = 0
DONT_PAUSE = false
CUES = {}

local function join_paths(path1, path2)
    if path2:match("^[A-Za-z]:[/\\]") then
        return path2
    end

    path1 = path1:gsub("[/\\]+$", "")
    path2 = path2:gsub("^[/\\]+", "")

    return path1 .. "/" .. path2
end

local function parent_folder(path)
    return path:match("^(.*[\\/])")
end

-- Converts a timestamp in `00:02:24:18` format to a time in seconds.
local function parse_timestamp(str)
    local hours, minutes, seconds, frame = str:match("(%d+)[:;](%d+)[:;](%d+)[:;](%d+)")
    local time = tonumber(hours) * 3600 + tonumber(minutes) * 60 + tonumber(seconds) + tonumber(frame) / FPS
    return time
end

local function current_cue()
    return (mp.get_property_native("chapter") or -1) + 1
end

local function next_cue()
    local slide = current_cue()
    if slide >= #CUES then
        local duration = mp.get_property("duration")
        mp.set_property_number("playback-time", duration)
    else
        mp.set_property_number("playback-time", CUES[slide + 1])
    end
end

local function previous_cue()
    local slide = current_cue()
    if slide <= 1 then
        mp.set_property_number("playback-time", 0)
    else
        mp.set_property_number("playback-time", CUES[slide - 1])
    end
end

local function on_file_loaded()
    FPS = mp.get_property_number("container-fps") or 60
    local cue_file = mp.get_opt("cue-file")

    -- Get the path of the cue file
    local path = join_paths(parent_folder(mp.get_property("path")), cue_file)
    local success, lines = pcall(io.lines, path)
    if not success then return end

    local chapter_list = mp.get_property_native("chapter-list")
    local n = 0

    for line in lines do
        local timestamp = parse_timestamp(line)
        table.insert(chapter_list, n + 1, { title = "", time = timestamp })
        table.insert(CUES, timestamp)
        n = n + 1
    end

    mp.set_property_native("chapter-list", chapter_list)
    mp.set_property_bool("pause", true)
    mp.msg.info(string.format("Loaded %d cues.", #CUES))
end

local function on_advance()
    if mp.get_property_bool("pause") then
        mp.set_property_bool("pause", false)
    else
        DONT_PAUSE = true
        next_cue()
    end
end

local function on_playback()
    local slide = (mp.get_property_native("chapter") or -1) + 1
    if slide >= #CUES then return end

    if slide ~= LAST_SLIDE then
        LAST_SLIDE = slide
        if not DONT_PAUSE then
            mp.set_property_bool("pause", true)
        end

        DONT_PAUSE = false
    end
end

local cue_file = mp.get_opt("cue-file")
if not cue_file then
    mp.msg.info("No cue file defined, skipping (--script-opt=cue-file=<file>)")
    return
end

mp.register_event("file-loaded", on_file_loaded)
mp.observe_property("playback-time", "number", on_playback)
mp.add_key_binding("space", on_advance)
mp.add_key_binding("left", previous_cue)
mp.add_key_binding("right", next_cue)

-- TODO: fix bug where last section is ignored
