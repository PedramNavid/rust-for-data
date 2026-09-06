.PHONY: build-release setup data serve benchmarks benchmarks-offline

build-release:
	cargo build --locked --release --manifest-path=./wxrs/Cargo.toml --target-dir=./wxrs/target

# Create the Python environment from the committed lockfile.
setup:
	cd wxpy && uv sync --locked

# Chapter 5 works on the Project FeederWatch dataset, which is checked in as a
# 7z archive because the extracted CSV is ~1.4GB. The extracted .csv is
# gitignored.
data: lib/PFW_2016_2020_public.csv

lib/PFW_2016_2020_public.csv:
	cd lib && uv run --locked --project ../wxpy --group data python -c \
		"import py7zr; py7zr.SevenZipFile('PFW_2016_2020_public.7z').extractall('.')"

serve:
	cd rust4data-book && make $@

# Everything, including the benchmarks that need an OpenWeather API key.
benchmarks:
	cd benchmarks && make all

# Just the benchmarks that run without network access or an API key.
benchmarks-offline:
	cd benchmarks && make offline
