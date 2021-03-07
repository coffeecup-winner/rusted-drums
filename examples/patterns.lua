pattern(D4, string.rep([[********]], 2))
pattern(D3, string.rep([[--*---*-]], 2))
pattern(C3, string.rep([[*---*---]], 2))

for i = 0, 15 do
    if i % 3 == 0 or i % 7 == 0 then
        play(i, B3, VELOCITY_DEFAULT)
    end
end
