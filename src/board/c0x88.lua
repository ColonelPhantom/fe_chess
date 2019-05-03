for rank=1,8 do
    for file=1,8 do
        sq0x88 = (16 * (rank-1)) + (file-1);
    
        print(
            "pub const "..
            string.char(file-1 + string.byte("a"))..rank..
            ": Coord0x88 = Wrapping(0x"..
            string.format("%02x", sq0x88)
            ..");"
        )
    end
end