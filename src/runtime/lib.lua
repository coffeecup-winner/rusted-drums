__events = {}

function note_on(tick, note, velocity)
    __events[tick] = {}
    __events[tick]['event'] = 'NOTE_ON'
    __events[tick]['note'] = note
    __events[tick]['velocity'] = velocity
end

function note_off(tick, note, velocity)
    __events[tick] = {}
    __events[tick]['event'] = 'NOTE_OFF'
    __events[tick]['note'] = note
    __events[tick]['velocity'] = velocity
end
