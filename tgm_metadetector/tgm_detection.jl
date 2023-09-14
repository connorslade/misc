### A Pluto.jl notebook ###
# v0.19.27

using Markdown
using InteractiveUtils

# ╔═╡ b7ab2d94-80bc-42b4-8665-8eb2d1565864
using Plots

# ╔═╡ a4323470-52a1-11ee-31e5-3b141b00dc84
begin
	global TGM_TEXT = "data/tgm.txt"
	global HUMAN_TEXT = "data/me.txt"

	function format_content(raw)
		replace(raw, '\n' => " ", '\r' => "")
	end
	
	global TGM_CONTENT = format_content(read(TGM_TEXT, String))
	global HUMAN_CONTENT = format_content(read(HUMAN_TEXT, String))
end

# ╔═╡ 19def73a-fbda-48ae-b228-5ae7fcaf01b4
md"## Plots"

# ╔═╡ 09287786-6235-4492-8b08-5a090d2e3e5f
md"### Line Length"

# ╔═╡ ab96449a-a570-4b22-b4df-a7a8e6e48b73
begin
	local function map_lines(x)
		local list = filter(x -> x > 1, map(x -> length(x), split(x, ".")));
		return list
	end
	
	local tgm_lines = map_lines(TGM_CONTENT)
	local human_lines = map_lines(HUMAN_CONTENT)

	histogram(tgm_lines, bins=10, label="TGM", color=:red)
	histogram!(human_lines, bins=10, label="Human", seriesalpha=0.5, color=:blue)
end

# ╔═╡ 923da5e8-ffb1-4ed7-82e5-b36b7f2a4efa
md"### Word Length"

# ╔═╡ 67701959-ad60-4e41-ab77-8495856a3844
begin
	function is_word(x)
		local CHARS = "QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm"
		x = replace(x, ':' => "", '(' => "", ')' => "", '\'' => "", '’' => "", '-' => "")
		for i in x
			if !occursin(i, CHARS)
				return false;
			end
		end

		return true;
	end
	
	function map_lines_word_length(x)
		local raw = replace(x, '.' => "", ',' => "", ';' => "", '?' => ".", '!' => ".");
		local list = map(x -> length(x), filter(is_word, split(raw, " ")));
		return list
	end
	
	local tgm_words = map_lines_word_length(TGM_CONTENT)
	local human_words = map_lines_word_length(HUMAN_CONTENT)

	histogram(tgm_words, bins=20, label="TGM", color=:red)
	histogram!(human_words, bins=10, label="Human", seriesalpha=0.5, color=:blue)
end

# ╔═╡ 00000000-0000-0000-0000-000000000001
PLUTO_PROJECT_TOML_CONTENTS = """
[deps]
Plots = "91a5bcdd-55d7-5caf-9e0b-520d859cae80"

[compat]
Plots = "~1.39.0"
"""

# ╔═╡ 00000000-0000-0000-0000-000000000002

# ╔═╡ Cell order:
# ╠═b7ab2d94-80bc-42b4-8665-8eb2d1565864
# ╠═a4323470-52a1-11ee-31e5-3b141b00dc84
# ╟─19def73a-fbda-48ae-b228-5ae7fcaf01b4
# ╟─09287786-6235-4492-8b08-5a090d2e3e5f
# ╠═ab96449a-a570-4b22-b4df-a7a8e6e48b73
# ╟─923da5e8-ffb1-4ed7-82e5-b36b7f2a4efa
# ╠═67701959-ad60-4e41-ab77-8495856a3844
# ╟─00000000-0000-0000-0000-000000000001
# ╟─00000000-0000-0000-0000-000000000002
