function r()
    return math.random(0, 9223372036854775807)
end


math.randomseed(os.time());

pieces = {"WPAWN", "WKNIGHT", "WBISHOP", "WROOK", "WQUEEN", "WKING", "BPAWN", "BKNIGHT", "BBISHOP", "BROOK", "BQUEEN", "BKING"}

for i,p in pairs(pieces) do
    io.write("pub const "..p..": [u64; 64] = [")
    for i = 1,64 do
        io.write(r()..", ");
    end
    print("];")
end

print("pub const SIDE_TO_MOVE: u64 = "..r()..";")

io.write("pub const CASTLING: [u64; 4] = [")
for i=1,4 do
    io.write(r()..", ");
end
print("];")

io.write("pub const ENPASSANT: [u64; 8] = [")
for i=1,8 do
    io.write(r()..", ");
end
print("];")
