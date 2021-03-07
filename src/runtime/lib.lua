__events = {}
__next_event_id = 1

function note_on(tick, note, velocity)
    __events[__next_event_id] = {
        tick = tick,
        event = NOTE_ON,
        note = note,
        velocity = velocity,
    }
    __next_event_id = __next_event_id + 1
end

function note_off(tick, note, velocity)
    __events[__next_event_id] = {
        tick = tick,
        event = NOTE_OFF,
        note = note,
        velocity = velocity,
    }
    __next_event_id = __next_event_id + 1
end

function play(tick, note, velocity)
    note_on(tick, note, velocity)
    note_off(tick + 1, note, VELOCITY_OFF)
end

function pattern(note, pattern_string)
    for i = 1, #pattern_string do
        local c = pattern_string:sub(i, i)
        if c == '*' then
            play(i - 1, note, VELOCITY_DEFAULT)
        elseif c == '-' then
            -- do nothing
        else
            error('Unknown pattern char \'' .. c .. '\'')
        end
    end
end
