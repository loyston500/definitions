echo 'Thank you for installing!'

echo 'Building...'
cargo build --release

echo 'Moving the file to /usr/local/bin. Need sudo permission'
sudo cp target/release/definitions /usr/local/bin/def # you can change `def` with whatever you want

######

# extracting and linking the dict file
cd dictionary

echo 'Extracting the dict file...'
xz -d *

echo 'Everything done!'
echo 'Now put this line below at the top of your .bashrc or config.fish file depending on the shell you use. Then restart the shell'
echo 
echo "export DICTIONARY_FILE=$(realpath websters_dictionary.txt)"
