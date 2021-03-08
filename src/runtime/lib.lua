-- Private global variables
__events = {}
__next_event_id = 1
__invoke_last_pattern = nil

-- MIDI note on
function note_on(tick, note, velocity)
    __events[__next_event_id] = {
        tick = tick,
        event = NOTE_ON,
        note = note,
        velocity = velocity,
    }
    __next_event_id = __next_event_id + 1
end

-- MIDI note off
function note_off(tick, note, velocity)
    __events[__next_event_id] = {
        tick = tick,
        event = NOTE_OFF,
        note = note,
        velocity = velocity,
    }
    __next_event_id = __next_event_id + 1
end

-- MIDI note on and off on the next tick
function play(tick, note, velocity)
    note_on(tick, note, velocity)
    note_off(tick + 1, note, VELOCITY_OFF)
end

-- Internal pattern function
function __pattern(note, start_time, pattern_string)
    for i = 1, #pattern_string do
        local c = pattern_string:sub(i, i)
        if c == '*' then
            play(start_time + i - 1, note, VELOCITY_DEFAULT)
        elseif c == '-' then
            -- do nothing
        else
            error('Unknown pattern char \'' .. c .. '\'')
        end
    end
    __invoke_last_pattern = function()
        __pattern(note, start_time + #pattern_string, pattern_string)
    end
end

-- Pattern object (callable as pattern:xyz)
pattern = {}

-- Load a keymap into the pattern object
function use_keymap(keymap_name)
    local keymap = __keymaps[keymap_name]
    for k, v in pairs(keymap) do
        pattern[k] = function(self, pattern_string)
            __pattern(v, 0, pattern_string)
        end
    end
end

-- x2..x32 functions for repetition
for i = 2, 32 do
    _ENV['x' .. i] = function()
        for _i = 2, i do
            __invoke_last_pattern()
        end
    end
end
