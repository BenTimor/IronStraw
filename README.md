# IronStraw - an Advanced Template Engine
IronStraw is a new, advanced and modern template engine which allows you to create HTML pages and XML files.
The template engine is command-line based for now, and I hope that one day I'll turn it into a server-side language.

## Installation
All you have to do is download the last release from the [Releases](https://github.com/BenTimor/IronStraw/releases) page.

## Usage
For rendering pages, all you have to do is create a '.sw' file and use the command:

Linux:

    ./IronStraw [file/directory] [file2/directory2]...
Windows:

    IronStraw.exe [file/directory] [file2/directory2]...
If you enter a directory, it'll render all the '.sw' files in that directory recursively.

### Options
--debug = Printing what's going behind the scenes

--target [path] = Creating the files in the target path

--XML [file] = Renders the file wihtout adding 'html' tag and 'doctype'.

## How to write a 'Straw' file
For the examples I'll sometimes use code blocks, like this:

    IronStraw Code => HTML Code

### HTML/XML tags
If you want to create a tag, all you have to do is name the tag.

    tag => <tag> </tag>
You can also add parameters, like this:

    tag(parm1, parm2) => <tag parm1, parm2> </tag>
And ofcourse, you can add text:

    tag(myparm) text => <tag myparm> text </tag>
If you want to create a tag inside a tag, you have to name the second tag in the first tag's block. In Straw, you can create a block by spacing the code.

Straw:

    tag
	    anothertag text
	    anothertag text2
HTML:

    <tag>
	    <anothertag> text </anothertag>
	    <anothertag> text2 </anothertag>
	</tag>

**Note:** Straw automatically adds html tag and doctype. If you don't want this to happen, you have to add --XML.

### Straw Commands
Straw has two types of commands, Processed Commands which start with '@' and Preprocessed Commands which start with '^'.
You use both of the commands in the same way:

    @command(parms) text
	    block line 1
		block line 2
		...

## Commands List

### @note

---
Allows you to create HTML notes (<!-- ->).

**Parameters:** The command ignores the parameters so you can put there whatever you want.
**Text and block:** The text and the block will be written inside the HTML note.
**Short Command:** # *Note*

---
### @straw_note

---
Ignores everything that you put inside. Allows you to write notes which won't be found in your HTML file.

**Short Command:** // *Note*

---
### @raw

---
Allows you to write 'raw' HTML/XML. It usually won't process any of its content.

**Parameters:** 
1. Render the 'text' after the command | Boolean (true/false) | Default true.

**Text:** Depends on the parameter. If it's true, it'll allow you to name a tag that you want to put the blocks inside. If it's false, the text will just appear on the XML/HTML file.

**Block:** Appear on the file as the are.

**Short Command:**
1. .*Raw* = @raw(false) Raw
2. **Raw* = @raw(true) Raw

**Examples:**

Straw:

    *script
	    alert(1);

HTML:

    <script>
	    alert(1);
	</script>
--
Straw:

    .@raw
XML:

    @raw

---
### @loop

---
Allows you to run specific code multiple times. It also renders the content.

**Parameters:**

1. Render the content of the file | Boolean | Default false

**Text:** The number of times you want it to run
**Block:** The code you want to run multiple times

---
### ^set

---
Replacing something in something else in all of the file. Allows you to create something like variables.

**Parameters:**

1. From - What you want to replace
2. To - What you want to put instead of the from

**Text and block:** None

**Example:**
Straw:

    ^set({site}, MySite)
    title Welcome to {site}!
HTML:

    <title> Welcome to MySite! </title>

---
### @terminal

---

Allows you to run terminal commands. It puts the output in the file.

**Parameters:** None
**Text:** The command you want to run
**Block:** None

---
### @file

---
Reading a file into your code.

**Parameters:**

1. Render the content of the file | Boolean | Default false

**Text:** Path
**Block:** None

---
### @intofile

---
Writing a content into a file.

**Parameters:**

1. Render the content of the file | Boolean | Default false

**Text:** Path
**Block:** The content you want to write into the file.

---
### @delfile

---
Deleting a file.

**Parameters:** None
**Text:** Path
**Block:** None