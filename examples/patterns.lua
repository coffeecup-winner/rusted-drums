use_keymap('AddictiveDrums2')

pattern:hh(string.rep([[********]], 2))
pattern:snare(string.rep([[--*---*-]], 2))
pattern:kick(string.rep([[*---*---]], 2))

for i = 0, 15 do
    if i % 3 == 0 or i % 7 == 0 then
        play(i, B3, VELOCITY_DEFAULT)
    end
end
