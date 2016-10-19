# requires ambr, part of the amber cargo package.  install with
# "cargo install amber"

elm-make Main.elm --output main.js

cp index.html.template meh1
ambr elm-main.js --rep-file main.js meh1 --no-interactive
cp ../src/string_defaults.rs.template ../src/string_defaults.rs
ambr index.html --rep-file meh1 ../src/string_defaults.rs --no-interactive

mv meh1 index.html+main.js

