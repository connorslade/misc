using JSON
using StatsPlots

local data = JSON.parsefile("test_score_scraper/data.json")

sat = []
act = []
gpa = []

function process_test(data)
    if isnothing(data)
        return nothing
    end

    local apps = data["apps"]
    for catagory in [
        "accepted",
        "denied",
        "waitlistedUnknown",
        "waitlistedAccepted",
        "waitlistedDenied"
    ]
        local students = apps[catagory]
        if isnothing(students)
            continue
        end

        for student in students
            for (list, index) in [(gpa, "gpa"), (sat, "highestComboSat"), (act, "actCompositeStudent")]
                local value = student[index]
                if isnothing(value) || value == 0
                    continue
                end
                push!(list, value)
            end
        end
    end
end


for (collage_id, college) in data
    local scattergrams = college[2]["scattergrams"]
    if isnothing(scattergrams)
        continue
    end

    local weighted_gpa = scattergrams["weightedGpa"]
    if isnothing(weighted_gpa)
        continue
    end

    println("Processing $(college[1]["name"])")
    process_test(weighted_gpa["sat"])
    process_test(weighted_gpa["act"])
end


println()
println("[*] Found $(length(sat)) SAT scores")
println("[*] Found $(length(act)) ACT scores")
println("[*] Found $(length(gpa)) GPAs")


plot(
    boxplot(["sat"], sat),
    boxplot(["act"], act),
    boxplot(["gpa"], gpa),
    size = (750, 500),
    layout = (1, 3)
)