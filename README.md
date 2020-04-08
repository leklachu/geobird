# Geobird

Gets a bird's-eye view of parts of the geoid.

## Status

INCOMPLETE.

Currently it's a library that can make a URL for an image.
- no config
- no fetching
- no putting

but at least it's a start.

## Function

Geobird uses the excellent resource provdided by NASA to view recent and
less-recent satellite photos of the earth, and gets images for a specific area,
collected by date and satellite.

## Binary targets

Geobird can... will be able to... run as a CLI app to batch-get images or as a
web service in Webassembly.

## History

I wanted to get a sequence of images that I could scroll through quickly in feh
to see the changing seasons and current snowfall. Maybe I tried first in BASH;
then Haskell (satellite-record) then ported to Go (geobird-go). This is my
latest port as I get keen on Rust.

After all, now if someone were ever to ask me to program/maintain in a
non-favourite language, I can reply, "I'm sorry, my programming skills are a
little rusty."

