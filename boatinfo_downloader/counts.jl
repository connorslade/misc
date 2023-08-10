using CSV
using DataFrames
using Plots

frame = CSV.read("./out.csv", DataFrame)
counts = Dict()

for i in eachrow(frame)
    name = replace(i[2], "&#39;" => "'")

    if haskey(counts, name)
        counts[name] += 1
    else
        counts[name] = 1
    end
end

sorted = sort(counts, byvalue=true, rev=true)
CSV.write("./counts.csv", sorted)