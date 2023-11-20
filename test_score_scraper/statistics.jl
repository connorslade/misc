using JSON
using StatsPlots

local data = JSON.parsefile("test_score_scraper/data.json")

function process_test(data, stats)
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
            for (list, index) in stats
                local value = student[index]
                if isnothing(value) || value == 0
                    continue
                end
                push!(list, value)
            end
        end
    end
end


function main()
    local sat = []
    local act = []
    local gpa = []

    local stats = [
        (gpa, "gpa"),
        (sat, "highestComboSat"),
        (act, "actCompositeStudent")
    ]

    for college in values(data)
        local scattergrams = college[2]["scattergrams"]
        if isnothing(scattergrams)
            continue
        end

        local weighted_gpa = scattergrams["weightedGpa"]
        if isnothing(weighted_gpa)
            continue
        end

        println("Processing $(college[1]["name"])")
        process_test(weighted_gpa["sat"], stats)
        process_test(weighted_gpa["act"], stats)
    end


    println()
    println("[*] Found $(length(sat)) SAT scores")
    println("[*] Found $(length(act)) ACT scores")
    println("[*] Found $(length(gpa)) GPAs")


    plot(
        boxplot(["sat"], sat),
        boxplot(["act"], act),
        boxplot(["gpa"], gpa),
        size=(1000, 750),
        layout=(1, 3)
    )
end

main()