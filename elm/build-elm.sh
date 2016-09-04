elm-make Main.elm --output main.js

cp index.html.template meh1
ambr elm-main.js --rep-file main.js meh1 --no-interactive
cp ../src/stringDefaults.rs.template ../src/stringDefaults.rs
ambr index.html --rep-file meh1 ../src/stringDefaults.rs --no-interactive

rm meh1


