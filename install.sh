echo 'Thank you for installing!'

echo 'Building...'
cargo build --release

echo 'Moving the file to /usr/local/bin. Need sudo permission'
sudo cp target/release/definitions /usr/local/bin/def # you can change `def` with whatever you want

######

# extracting and linking the dict file
cd dictionary

echo 'Extracing the dict file...'
xz -d *
export DICTIONARY_FILE="$(realpath websters_dictionary.txt)"

echo 'Everything done!'