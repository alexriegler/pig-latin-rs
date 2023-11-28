# Pig Latin Converter

This a project inspired by an exercise listed in [*The Rust Programming Language*](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#summary).

Excerpt from chapter 8 section 3:

```
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
```

Example run:

```
> ./pig-latin.exe
Welcome to the Pig Latin converter!
Please enter a phrase to convert.
first
irst-fay
Please enter a phrase to convert.
apple
pple-hay
Please enter a phrase to convert.
quit
Thanks for using the Pig Latin converter!
```
