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

note_on(0, C3, VELOCITY_DEFAULT)
note_off(1, C3, VELOCITY_OFF)
note_on(2, D3, VELOCITY_DEFAULT)
note_off(3, D3, VELOCITY_OFF)
note_on(4, C3, VELOCITY_DEFAULT)
note_off(5, C3, VELOCITY_OFF)
note_on(6, D3, VELOCITY_DEFAULT)
note_off(7, D3, VELOCITY_OFF)
