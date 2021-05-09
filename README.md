# Definitions  
A simple dictionary for you.  

## Usage  
Easy, just do this thing below and you get your definition.  
```py  
def assemble  
```  
Output:  
```bash  
To collect into one place or body; to bring or call together; to convene; to congregate. Thither he assembled all his train. Milton. All the men of Israel assembled themselves. 1 Kings viii.  
2.  
```  
  
  
It also does *fuzzy* searching, just in case you f\*ck up while typing.  
```py  
def assembel  
```  
Output:  
```bash  
Did you mean: assemble, assembly, assembler, assemblage, reassemble, assemblance, assemblyman, assume, passee, semble ?  
  
Giving definition of the closest match..  
  
assemble -  
  
To collect into one place or body; to bring or call together; to convene; to congregate. Thither he assembled all his train. Milton. All the men of Israel assembled themselves. 1 Kings viii.  
2.  
```  
  
  
It also does *coloring*.  
  
![lmao](https://media.discordapp.net/attachments/723907168184565830/840614451571851314/what_you_want.png)  
  
## Source
Where you get the dictionary file from?
Here, https://github.com/matthewreagan/WebstersEnglishDictionary  
I just took it, parsed it, compressed it to save your precious download time, and put it in `definitions/dictionary`

## Installation
Installation is pretty easy, you need rust (for building), bash and xz (you probably have them already)
```bash
git clone https://github.com/loyston500/definitions
cd definitions
bash install.sh
```

Or do everything manually..
```bash
git clone https://github.com/loyston500/definitions
cd definitions
cargo build --release
sudo cp target/release/definitions /usr/local/bin/def # you can change `def` with whatever you want

# extracting and linking the dict file
cd dictionary
xz -d *
export DICTIONARY_FILE="$(realpath websters_dictionary.txt)"
```
