using Dates
using Serialization
using Images
using FileIO
using Plots

IMAGE_PATH = "C:\\Users\\turtl\\Documents\\ShareX\\Screenshots"
BIN_PATH = "screenshots.bin"

struct Image
    path::String
    pixel_count::Int
    created::DateTime
end

images = Vector{Image}()

if isfile(BIN_PATH)
    images = deserialize(BIN_PATH)
    println("Loaded $(length(images)) images from bin")
else
    println("Loading images from disk...")
    for (root, dirs, files) in walkdir(IMAGE_PATH)
        old_count = length(images)
        print("Processing $root ")
        for file in files
            if !endswith(file, ".png") && !endswith(file, ".jpg")
                continue
            end

            path = joinpath(root, file)

            try
                img = load(path)
                pixel_count = size(img, 1) * size(img, 2)
                created = Dates.DateTime(Dates.unix2datetime(stat(path).mtime))
                push!(images, Image(path, pixel_count, created))
            catch
                println("Error loading $path")
                continue
            end
        end

        println("($(length(images) - old_count))")
    end

    println("Total images: $(length(images))")
    sort!(images, by=x -> x.created)

    serialize(BIN_PATH, images)
end

sort!(images, by=x -> x.created)

# Plot the number of screenshots for each month
months = Dict{String,Int64}()

for image in images
    month = Dates.format(image.created, "yyyy-mm")
    if !haskey(months, month)
        months[month] = 0
    end

    months[month] += 1
end

months = sort(collect(months), by=x -> x[1])
x = [x[1] for x in months]
y = [x[2] for x in months]

# Plot the number of screenshots each year
years = Dict{Int64,Int64}()

for image in images
    year = Dates.year(image.created)
    if !haskey(years, year)
        years[year] = 0
    end

    years[year] += 1
end

x2 = collect(keys(years))
y2 = collect(values(years))

plot(bar(x, y), bar(x2, y2), size=(2000, 1000))