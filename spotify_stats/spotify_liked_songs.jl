### A Pluto.jl notebook ###
# v0.19.27

#> [frontmatter]
#> title = "Spotify Liked Songs Analysis"
#> date = "2023-08-14"

using Markdown
using InteractiveUtils

# ╔═╡ 203245d0-3a59-11ee-172d-85f8475d0b24
begin
	using Dates
	using JSON
	using Serialization
	using PlutoUI
	using Plots
	using StatsBase
end

# ╔═╡ bdb5fd8b-a673-4515-b57d-bdf181312398
md"# Spotify Liked Songs Analysis"

# ╔═╡ b857b9ed-4761-4a18-abb4-bcafdeaf18dc
TableOfContents()

# ╔═╡ ce486027-6e3d-45dc-af45-e193f7ac69c8
md"## Define Some Stuff"

# ╔═╡ 14ea412d-25ee-4aed-9735-6f1eebb88d3c
PLAYLIST_BACKUP = "V:\\Programming\\Git\\spotify_song_sanctuary\\spotify_backup_2023-08-14.json"

# ╔═╡ 8a1eddfd-dbea-405f-9712-fb9c137329e1
BIN_PATH = "V:\\Programming\\Projects\\misc\\playlist.bin"

# ╔═╡ 44ed90cb-39a5-4486-8c57-ba3f50335992
struct Song
    name::String
	artists::Vector{String}
    id::String
    added::DateTime
    duration_ms::Int64
    explicit::Bool
end

# ╔═╡ 00a50c59-575e-4be1-aacf-7fff83fb2942
md"## Load Liked Songs
Will either load from a .json file and output a cache file, or read the cache file directly."

# ╔═╡ 77275bb2-5737-4574-85b9-030b5027ee50
begin
	global songs = Vector{Song}()
		
	if isfile(BIN_PATH)
		println("Loading liked songs from cache...")
		global songs = deserialize(BIN_PATH)
	else
		println("Loading liked songs from disk...")
		local raw_playlist = JSON.parsefile(PLAYLIST_BACKUP)
	
		for i in raw_playlist
			local track = i["track"]

			local artists = Vector{String}()
			for i in track["artists"]
				push!(artists, i["name"])
			end
			
			local song = Song(
				track["name"],
				artists,
				track["id"],
				Dates.DateTime(i["added_at"], "yyyy-mm-ddTHH:MM:SSZ"),
				track["duration_ms"],
				track["explicit"]
			)
			push!(songs, song)
		end
	
		serialize(BIN_PATH, songs)
	end
end

# ╔═╡ c98868ff-e13b-441a-9e6a-13691c522f19
md"## Plots"

# ╔═╡ 9b239d53-0c93-49d7-bdf8-91327f748b98
md"## Songs Over Time"

# ╔═╡ 09e79c05-5da6-4780-9d05-6cb912f095f7
begin
	local days = Vector{Tuple{DateTime, Int64}}()
	local inc = 0

	for i in reverse(songs)
		inc += 1
		push!(days, (i.added, inc))
	end

	plot(days)
end

# ╔═╡ 8c5c238a-934a-4571-a5ad-16d269e33014
md"## Songs Added per Month"

# ╔═╡ 72defedb-bf2b-49a1-bbd1-41e9db171f77
begin
	local months = Dict{String, Int64}()
	
	for i in songs
		month = Dates.month(i.added)
		year = Dates.year(i.added)
		key = "$year-$month"
	
		if !haskey(months, key)
			months[key] = 0
		end
	
		months[key] += 1
	end
	
	local months = sort(collect(months), by=x -> x[1])
	local x = [x[1] for x in months]
	local y = [x[2] for x in months]

	bar(x, y)
end

# ╔═╡ 57abe789-60b9-4766-9783-b556b2f90cf2
md"## Songs Added per Year"

# ╔═╡ fb9bb8ec-5329-4828-8ce0-528cde95846e
begin
	local years = Dict{Int16, Int64}()
	
	for i in songs
		year = Dates.year(i.added)
	
		if !haskey(years, year)
			years[year] = 0
		end
	
		years[year] += 1
	end
	
	local years = sort(collect(years), by=x -> x[1])
	local x = [x[1] for x in years]
	local y = [x[2] for x in years]

	bar(x, y)
end

# ╔═╡ 71d66ee5-a627-4c2c-8168-3c63d179f168
md"## Songs Added per Day of the Week"

# ╔═╡ 53f00870-dccd-4572-9f8f-b12a76961940
begin
	const DAYS = [
		"Monday",
		"Tuesday",
		"Wednesday",
		"Thursday",
		"Friday",
		"Saturday",
		"Sunday"
	]
	
	local days = Dict{Int64, Int64}()
	
	for i in songs
		day = Dates.dayofweek(i.added)
	
		if !haskey(days, day)
			days[day] = 0
		end
	
		days[day] += 1
	end
	
	days = sort(collect(days), by=x -> x[1])
	x = [DAYS[x[1]] for x in days]
	y = [x[2] for x in days]

	bar(x, y)
end

# ╔═╡ e6cfa69e-d7ac-48b1-aa1b-4d90ead200a3
md"## Songs that are Explicit"

# ╔═╡ ed845056-fd47-4e69-bb47-c3f599b397be
begin
	local total = length(songs);
	local explicit = 0;
	
	for i in songs
		if i.explicit
			explicit += 1;
		end
	end

	println("$(round(explicit / total * 1000) / 10)% explicit")
	pie(["Explicit", "Not-Explicit"], [explicit, total - explicit])
end

# ╔═╡ f08d9d85-b26d-4a7d-a6f3-94100ad2b22d
md"## Average Length of Songs Added Each Year "

# ╔═╡ 4a89375d-293c-4099-9738-b4c886196c0c
begin
	local years = Dict{Int16, Tuple{Int64, Int64}}()
	
	for i in songs
		local year = Dates.year(i.added)
		local length = floor(i.duration_ms / 1000)
		
		if !haskey(years, year)
			years[year] = (0, 0)
		end
	
		local old = years[year]
		years[year] = (old[1] + length, old[2] + 1)
	end
	
	local years = sort(collect(years), by=x -> x[1])
	local x = [x[1] for x in years]
	local y = [x[2][1] / x[2][2] for x in years]

	bar(x, y)
end

# ╔═╡ 6113e557-470a-4763-8e10-d304169fd786
md"## Duplacate Songs"

# ╔═╡ 48c5d1cc-0b2e-4c1a-801f-3f3b27a8825e
begin
	local counts = countmap([(x.name, x.artists) for x in songs])
	local items = Vector{Tuple{Tuple{String, Vector{String}}, Int64}}()

	for i in collect(counts)
		if i[2] > 1
			push!(items, ((i[1][1], i[1][2]), i[2]))
		end
	end

	sort!(items, by=x->x[2], rev=true)
	for i in items
		println("$(i[1][1]) $(repeat(" ", 30 - length(i[1][1])))- $(i[2])")
	end
end

# ╔═╡ 00000000-0000-0000-0000-000000000001
PLUTO_PROJECT_TOML_CONTENTS = """
[deps]
Dates = "ade2ca70-3891-5945-98fb-dc099432e06a"
JSON = "682c06a0-de6a-54ab-a142-c8b1cf79cde6"
Plots = "91a5bcdd-55d7-5caf-9e0b-520d859cae80"
PlutoUI = "7f904dfe-b85e-4ff6-b463-dae2292396a8"
Serialization = "9e88b42a-f829-5b0c-bbe9-9e923198166b"
StatsBase = "2913bbd2-ae8a-5f71-8c99-4fb6c76f3a91"

[compat]
JSON = "~0.21.4"
Plots = "~1.38.17"
PlutoUI = "~0.7.52"
StatsBase = "~0.34.0"
"""

# ╔═╡ Cell order:
# ╟─bdb5fd8b-a673-4515-b57d-bdf181312398
# ╟─b857b9ed-4761-4a18-abb4-bcafdeaf18dc
# ╠═203245d0-3a59-11ee-172d-85f8475d0b24
# ╟─ce486027-6e3d-45dc-af45-e193f7ac69c8
# ╠═14ea412d-25ee-4aed-9735-6f1eebb88d3c
# ╠═8a1eddfd-dbea-405f-9712-fb9c137329e1
# ╠═44ed90cb-39a5-4486-8c57-ba3f50335992
# ╟─00a50c59-575e-4be1-aacf-7fff83fb2942
# ╠═77275bb2-5737-4574-85b9-030b5027ee50
# ╟─c98868ff-e13b-441a-9e6a-13691c522f19
# ╟─9b239d53-0c93-49d7-bdf8-91327f748b98
# ╟─09e79c05-5da6-4780-9d05-6cb912f095f7
# ╟─8c5c238a-934a-4571-a5ad-16d269e33014
# ╟─72defedb-bf2b-49a1-bbd1-41e9db171f77
# ╟─57abe789-60b9-4766-9783-b556b2f90cf2
# ╟─fb9bb8ec-5329-4828-8ce0-528cde95846e
# ╟─71d66ee5-a627-4c2c-8168-3c63d179f168
# ╟─53f00870-dccd-4572-9f8f-b12a76961940
# ╟─e6cfa69e-d7ac-48b1-aa1b-4d90ead200a3
# ╟─ed845056-fd47-4e69-bb47-c3f599b397be
# ╟─f08d9d85-b26d-4a7d-a6f3-94100ad2b22d
# ╟─4a89375d-293c-4099-9738-b4c886196c0c
# ╟─6113e557-470a-4763-8e10-d304169fd786
# ╠═48c5d1cc-0b2e-4c1a-801f-3f3b27a8825e
# ╟─00000000-0000-0000-0000-000000000001
# ╟─00000000-0000-0000-0000-000000000002
