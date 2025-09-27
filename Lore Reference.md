The Lore language aims for extreme abstraction. Its functions, such as representing a novel's table of contents or webpage bookmarks, are determined by its implementation rather than its syntax.

Lore files use the `.l` extension.

# Lines

Each line is an independent object with an indent level.

As a reference or not? With a scope or Not? However, each of the two questions can be ignored.

With 9 directly line types and 2 implementation line types, there are only 11 types of lines in the Lore language.

因为我编程水平很菜然后现在大一刚开学我还要考四六级所以没时间天天敲rust，so there are only 4 of them implemented yet.

## 11 Line Types(4 of them implemented)

### Atom Type Lines

It is like a map.

```lore
[ * ] = [ * ]
```

For example,

```lore-wcp
GitHub = https://www.github.com
```

Connect two literal measures as an Atom line. TODO, some escape method.

### Domain Type Lines

Being with a scope, It is like a floder.

```lore
+ [ * ]
  ...a sub-domain with enhanced indents
```

For example,

```lore-wcp
+ website collections
  GitHub = https://www.github.com
  Bilibili = https://www.bilibili.com
  ...
```

Then, make a mixture, as a new domain type, which is with a map value.

```lore-bs
+ Lore Reference = (the url of this book)
  (the index of chapters)
```

### Element Type Lines

When a line is not identified as any one of Lore Line types, it is an undefined Element Line.

```lore
[ * ]
```

For example, the comments which showed before.
