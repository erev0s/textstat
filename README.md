## text file statistics program in Rust  
The goal of this part is to implement a simple stand-alone program that will
allow you to work with the (zero-cost) abstractions provided by Rust and its
standard library. Your task is to write a program that, given a filename of a
text file, produces statistics (see below) for this text file.  
### Some guidelines and additional instructions:

- You can use cargo to create a template to start from:
cargo new textstat --bin
- Allow the text file to read to be specified on the command line, that is:
textstat file.txt
- Ignore case. “The”, and “the” should be considered the same word.
- Ignore punctuation marks while looking for words, however do consider
contractions (e.g., “I’m”) to be one word. The text “It’s a beautiful day.”
has the words “it’s”, “a”, “beautiful”, and “day”.  

#### The statistics should include at least:
- The total number of words.
- The average word size.
- A list of the number of words per word size (up to at least 10 char-
acters)
- A (sorted) list of the 10 most used words.  

Try to use the collections data structures of the Rust standard library 3
where possible (e.g., vectors, maps).
Also consider using iterators and consumers with the collections data
structures as they allow for very concise code
