
**jgrep** searches for _selector patterns_ in JSON input and prints each JSON object that matches the pattern.

* * *

# **SYNOPSIS**

**jgrep** [**−cfimnqv**] [**−e**_pattern_] [**−f **_file_] [**−−colour**] [_pattern_]

# **DESCRIPTION**

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

## **SELECTOR PATTERNS**

_Selector Patterns_ are a way of describing a JSON structure which **jgrep** can then use to try and _match_ to the structure of JSON input from within a specified file or piped input. If **jgrep** finds that the input JSON contains the structure specified by the _selector pattern_ then the input is considered _matched_ and will be printed out.

For example, suppose you consume the Nasa API which returns a JSON dataset describing _How Many People Are In Space Right Now_...
```bash
$ curl 'http://api.open-notify.org/astros.json'
```

...you might get a response resembling this:

```json
{
	"people": [{
		"name": "Oleg Kononenko",
		"craft": "ISS"
	}, {
		"name": "David Saint-Jacques",
		"craft": "ISS"
	}, {
		"name": "Anne McClain",
		"craft": "ISS"
	}],
	"number": 3
}
```

If you wish to use the _cli_ to verify that there are people in space, you can use **jgrep** to ensure there is a _people_ property on the object.
For that we can use the _property matcher_ which matches any JSON object who has the specified property on it.

```bash
$ curl 'http://api.open-notify.org/astros.json' | jgrep '.people'
```

If you do so you'll see that the _cli_ prints out the JSON object, because it matches.

Suppose though you learn that Nasa always return the _people_ property, but it will be an empty array. Luckily, _patterns_ can be chained to describe complex structures deep within the provided input. Instead we'll ensure the people array contains an object.

```bash
$ curl 'http://api.open-notify.org/astros.json' | jgrep '.people[.]'
```

The JSON output is still getting matched, so this is pretty cool, but then you realise there's another issue which is that the _identity_ matcher, which matches any JSON value, can also match _Null_.

To avoid mistaking _Null_ for a real astronaut, lets ensure that the response contains a proper JSON object with a _name_:

```bash
$ curl 'http://api.open-notify.org/astros.json' | jgrep '.people[.name]'
```

There, that should do it.

If _curl_-ing the _API_ doesn't return a match, we'll know that no people are in space. Lets hope that never happens.

# **Pattern Syntax**

**Identity**: `.`

This is the most straight forward matcher, as it matches _anything_ and _everything_. This is useful for printing out all valid JSON object in the input or matching against a non-empty sequence in a JSON array, such as `.people[.]`.

**Property**: `.prop_name`

This matcher matches against any JSON object which _has_ a property named as the specified property in the _pattern_. So, for example, in the above _pattern_, any object which has a property with the name "_prop\_name_" will be matched against.

**Property & Value**: `{"prop_name":"prop_value"}`

This matcher is the same as the _property_ pattern, except it also allows us to specify the expected value. So, for example, in the above _pattern_, any object which has a property with the name "_prop\_name_" whose value is the string _prop\_value_ will be matched against.

The value of a property can be any valid JSON primitive, which means it can be a _String_, _Number_, _Boolean_ or _Null_.

_String Value Matchers_:

There are several ways to match against the value of a string and these are loosely based on the [CSS Attribute Selector](https://developer.mozilla.org/en-US/docs/Web/CSS/Attribute_selectors) syntax.

_Property Exact Value_: `{"prop_name":"prop_value"}`

Matches JSON objects whose specified property has the exact specified value. 
For example: ``` {"prop_name":"prop_value"} ```

_Property Contains Exact Value_: `{"prop_name"~:"prop_value"}`

Matches JSON objects whose specified property contains the specified value as a _word_. A _word_ is a single part pf a whitespace-separated list of words.
 For example: ``` {"prop_name":"This value will surely contain prop_value in it."} ```

_Property Prefixed Value_: `{"prop_name"^:"prop_value"}`

Matches JSON objects whose specified property begins with the specified value. 
 For example: ``` {"prop_name":"prop_value is what I'm all about."} ```

_Property Suffixed Value_: `{"prop_name"$:"prop_value"}`

Matches JSON objects whose specified property ends with the specified value. 
 For example: ``` {"prop_name":"Know what I'm all about? It's prop_value"} ```

_Property Contains Value_: `{"prop_name"*:"prop_value"}`

Matches JSON objects whose specified property contains the specified value. 
 For example: ``` {"prop_name":"Wildcard search for a the 'prop_value' is awesome"} ```

**Array Index**: `[2]`

This matcher matches against an array by verifying that it contains a value at the specified index.

For example: ``` ["some different value","member_value","some other value"] ```

**Array & Value**: `[="member_value"]`

This matcher matches against an array by verifying that it contains the exact string "_member\_value_" inside it.

Just like the property matcher, the value of an array member can be any valid JSON primitive, which means it can be a _String_, _Number_, _Boolean_ or _Null_.

_Array Value Matchers_:

There are several ways to match against the value in an array and these are similar to the _property_ selector's _string value matchers_ but slightly different due to differences between String and Array.

_Array Exact Value_: `[="member_value"]`

Matches JSON array whose contains the exact specified value and that is the only single value in the array. This can be used with all _primitives_.
For example: ``` ["member_value"] ```

_Array Contains Exact Value_: `[~="member_value"]`

Matches JSON array whose contains the exact specified value.  This can be used with all _primitives_.
For example: ``` ["some different value","member_value","some other value"] ```

_Array Prefixed Value_: `[^="member_value"]`

Matches JSON array which contains a value which _begins_ with the specified value.  This can only be used with strings.
For example: ``` ["some different value","member_value is cool","some other value"] ```

_Array Suffixed Value_: `[$="member_value"]`

Matches JSON array which contains a value which _ends_ with the specified value.  This can only be used with strings.
For example: ``` ["some different value","Know what's cool? member_value","some other value"] ```

_Array Contains Value_: `[*="_value"]`

Matches JSON array whose contains the specified value as a substring of a value in the array. 
For example: ``` ["Know what's cool? wildcard search of a member_value value","some other value"] ```


## **EXIT CODES**

In line with _grep_, the **jgrep** exit codes returns the exit status 0 if a selector match is found in the file and 1 if no selector is matched.

## **INSTALLATION**

While we iron out the kinks we're avoiding publishing **jgrep** into public package managers.
That said, under the _releases_ tab you will find both an _OSX_ and _Linux_ release.

If you're on _OSX_ and use _Homebrew_ you can use the following script to install the latest OSX release:

```
brew install https://raw.githubusercontent.com/gmmorris/jgrep/master/packaging/homebrew.rb
```

## **EXAMPLES**

To find all JSON input with an object with property `name` on it:

```bash
$ jgrep '.name' myfile
```

To find all JSON input with an object with property `fellowship`, whose value is an array and in that array there's a JSON object with the property `name` on it whose value is `Aragorn`:

```bash
$ jgrep '.fellowship[{"name":"Aragorn"}]' myfile
```

To find all JSON input with whose root object has the property `name` on it:

```bash
$ jgrep -^ '.name' myfile
```

To find all JSON input which does not have the property `name` anywhere in its structure:

```bash
$ grep -v '.name' myfile
```

* * *