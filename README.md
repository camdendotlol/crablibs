# Crablibs - A fill-in-the-blanks word game

Crablibs is based on the classic [Mad Libs](https://en.wikipedia.org/wiki/Mad_Libs) game. Give it a template file, and it will ask you for a list of words. At the end, the words you input will be displayed as part of a humorous story!

## How to run

In your terminal, ``cd`` into the folder containing the ``crablibs`` binary. Create or download a template file (see below).

The program accepts a template file name as an arguments. For example:

``./crablibs template_file.txt``

This will make Crablibs take text and prompts from a file called ``template_file.txt``.

## The template language

Crablibs uses a very simple template language. Everything in curly braces - ``{`` and ``}`` - is treated as a fill-in-the-blank. Whatever is typed inside the braces will be given to the user as a prompt.

For example, a template might look like:

``
Stocks surged today as the {capitalized noun} Corporation announced a new {noun}. Their {singular occupation} claims that the new product runs more {adverb} than any of its competitors. During the announcement ceremony, people {past tense verb} to get their {plural noun} on the latest gadget.
``

When you provide this template to the program, you will be prompted to input each term (example input is shown):

```
capitalized noun: Potato
noun: pizza
singular occupation: plumber
adverb: happily
past tense verb: crawled
plural noun: crabs
```

At the end, the story is output as:

``
Stocks surged today as the Potato Corporation announced a new pizza. Their plumber claims that the new product runs more happily than any of its competitors. During the announcement ceremony, people crawled to get their crabs on the latest gadget.
``

You can put anything inside the brackets, but the tradition is to use only a [part of speech](https://www.butte.edu/departments/cas/tipsheets/grammar/parts_of_speech.html) and any details it needs to fit grammatically in the sentence. If you are too specific, the end result might be a little boring.

## Building from source

This project doesn't use any dependencies, so building should be straightforward if you've used Rust before.

Make sure the [Rust development environment](https://www.rust-lang.org/learn/get-started) is installed.

``git clone`` the repository to a chosen location, and run ``cargo build --release`` to compile a production-ready binary.
