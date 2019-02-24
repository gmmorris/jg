
**jgrep** — jgrep searches for _selector patterns_ in JSON input and prints each JSON object that matches the pattern.

* * *

**SYNOPSIS**

**jgrep** [**−cfimnqv**] [**−e**_pattern_] [**−f **_file_] [**−−colour**] [_pattern_]

**DESCRIPTION**

The **jgrep** utility searches any given input files, selecting lines that correctly parse as valid JSON and match one or more _selector patterns_.

The following options are available:

**−c**, **−−count**

Only a count of selected lines is written to standard output.

<!-- **−−colour**

Mark up the matching text with the expression stored in GREP_COLOR environment variable. The possible values of when can be ‘never', ‘always' or ‘auto'. -->

**−e** _pattern_

Specify a _selector pattern_ used during the search of the JSON input: an input line is selected if it parses as valid JSON and matches any of the specified _selector patterns_. This option is most useful when multiple **−e** options are used to specify multiple patterns.

**−−help**

Print a brief help message.

**−i**, **−−ignore-case**

Perform case insensitive matching. By default, **jgrep** is case sensitive.

**−m** _num,_ **−−max-count**=_num_

Stop reading the file after _num_ matches.

**−n**, **−−line-number**

Each output line is preceded by its relative line number in the file, starting at line 1.

**−q**, **−−quiet**, **−−silent**

Quiet mode: suppress normal output. **jgrep** will only search a file until a match has been found, making searches potentially less expensive.
This is useful if you're trying to ensure a certain match is present in the file and can rely on the Exit Code to get the result. _See **Exit Codes** section_

**−v**, **−−invert-match**

Selected lines are those _not_ matching any of the specified selector patterns.

**SELECTOR PATTERNS**



**EXIT CODES**

In line with _grep_, the **jgrep** exit codes returns the exit status 0 if a selector match is found in the file and 1 if no selector is matched.

**EXAMPLES**

To find all JSON input with an object with property `name` on it:

$ jgrep '.name' myfile

To find all JSON input with an object with property `fellowship`, whose value is an array and in that array there's a JSON object with the property `name` on it whose value is `Aragorn`:

$ jgrep '.fellowship[{"name":"Aragorn"}]' myfile

To find all JSON input with whose root object has the property `name` on it:

$ grep -^ '.name' myfile

To find all JSON input which does not have the property `name` anywhere in its structure:

$ grep -v '.name' myfile

* * *