## This was only needed bcause I forgot to escape quotes in the original data ##
## This is not needed anymore                                                 ##

FILE = "out.csv"
FILE_OUT = "out2.csv"

data = readlines(FILE)
header = data[1]
data = data[2:end]

out = String[]
for line in data
    parts = split(line, ",")
    len = length(parts)

    video_length = parts[len]
    watch_count = parts[len-1]
    id = parts[len-2]
    name = join(parts[1:len-3], ",")
    name = replace(name, "\"" => "\"\"")

    push!(out, "\"$name\",$id,$watch_count,$video_length")
end

write(FILE_OUT, header * "\n" * join(out, "\n"))