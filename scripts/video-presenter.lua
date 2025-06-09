-- A MPV plugin for using videos as fancy slide show presentations.
--
-- Deprecates my old video-presenter project that used libmpv at
-- https://github.com/connorslade/video-presenter.
--
-- Uses cuepoints put into a video edited with Premiere Pro or After Effects.
-- Then when playing back it will wait at the cuepoints for the space button to
-- be pressed. This will let use use normal videos for presentations allowing
-- for more advanced graphics and animations, while still allowing you to keep
-- perfect timing.

FPS = nil          -- FPS of the video container. Videos with variable FPS are not supported.
LAST_SLIDE = 0     -- The slide that the player was on last frame.
DONT_PAUSE = false -- Used to stop the `on_playback` hook from pausing when skiping the end of a slide
CUES = {}          -- List of cuepoints, each with a time and loop property

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

-- Parses a cuepoint defintion from the following format, where the loop
-- component is optional: `00:02:24:18 loop:00:02:23:00`
local function parse_cuepoint(str)
    local time = str:match("([%d:;]*)")
    local loop = str:match("loop:([%d:;]*)")

    return {
        time = parse_timestamp(time),
        loop = loop and parse_timestamp(loop) or nil
    }
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
        mp.set_property_number("playback-time", CUES[slide + 1].time)
    end
end

local function previous_cue()
    local slide = current_cue()
    local time = mp.get_property_number("playback-time")
    local paused = mp.get_property_bool("pause")
    mp.set_property_bool("pause", true)

    if time - 1 /  FPS > CUES[#CUES].time then
        mp.set_property_number("playback-time", CUES[#CUES].time)
        return
    end

    if paused then
        slide = slide - 1
    end

    if slide == 0 then
        mp.set_property_number("playback-time", 0)
    else
        mp.set_property_number("playback-time", CUES[slide].time)
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
        local cue = parse_cuepoint(line)
        table.insert(chapter_list, n + 1, { title = "", time = cue.time })
        table.insert(CUES, cue)
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
    local slide = current_cue()
    if slide ~= LAST_SLIDE then
        local loop = slide >= 1 and slide <= #CUES and CUES[slide].loop
        if loop and not DONT_PAUSE then
            mp.set_property_number("playback-time", loop)
            return
        end

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
