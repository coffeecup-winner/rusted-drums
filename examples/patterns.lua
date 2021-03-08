use_keymap('AddictiveDrums2')

pattern:hh      [[********]]; x2()
pattern:snare   [[--*---*-]]; x2()
pattern:kick    [[*---*---]]; x2()

for i = 0, 15 do
    if i % 3 == 0 or i % 7 == 0 then
        play(i, B3, VELOCITY_DEFAULT)
    end
end
