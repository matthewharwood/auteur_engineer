# JavaScript Style Guide

## 1 Introduction

This document serves as the **complete** definition of coding standards for source code in the JavaScript programming language. A JavaScript source file is described as being *in proper style* if and only if it adheres to the rules herein.

Like other programming style guides, the issues covered span not only aesthetic issues of formatting, but other types of conventions or coding standards as well. However, this document focuses primarily on the hard-and-fast rules that we follow universally, and avoids giving advice that isn't clearly enforceable (whether by human or tool).

### 1.1 Terminology notes

In this document, unless otherwise clarified:

1. The term *comment* always refers to *implementation* comments.
2. This Style Guide uses RFC 2119 terminology when using the phrases *must*, *must not*, *should*, *should not*, and *may*. The terms *prefer* and *avoid* correspond to *should* and *should not*, respectively. Imperative and declarative statements are prescriptive and correspond to *must*.

Other "terminology notes" will appear occasionally throughout the document.

### 1.2 Guide notes

Example code in this document is **non-normative**. That is, while the examples are in proper style, they may not illustrate the *only* stylish way to represent the code. Optional formatting choices made in examples must not be enforced as rules.

## 2 Source file basics

### 2.1 File name

File names must be all lowercase and may include underscores (`_`) or dashes (`-`), but no additional punctuation. Follow the convention that your project uses. Filenames' extension must be `.js`.

### 2.2 File encoding: UTF-8

Source files are encoded in **UTF-8**.

### 2.3 Special characters

#### 2.3.1 Whitespace characters

Aside from the line terminator sequence, the ASCII horizontal space character (0x20) is the only whitespace character that appears anywhere in a source file. This implies that:

1. All other whitespace characters in string literals are escaped, and
2. Tab characters are **not** used for indentation.

#### 2.3.2 Special escape sequences

For any character that has a special escape sequence (`\'`, `\"`, `\\`, `\b`, `\f`, `\n`, `\r`, `\t`, `\v`), that sequence is used rather than the corresponding numeric escape (e.g `\x0a`, `\u000a`, or `\u{a}`). Legacy octal escapes are never used.

#### 2.3.3 Non-ASCII characters

For the remaining non-ASCII characters, either the actual Unicode character (e.g. `∞`) or the equivalent hex or Unicode escape (e.g. `\u221e`) is used, depending only on which makes the code **easier to read and understand**.

```js
/* Best: perfectly clear even without a comment. */
const units = 'μs';

/* Allowed: but unnecessary as μ is a printable character. */
const units = '\u03bcs'; // 'μs'

/* Good: use escapes for non-printable characters with a comment for clarity. */
return '\ufeff' + content;  // Prepend a byte order mark.
```

Bad:
```js
/* Poor: the reader has no idea what character this is. */
const units = '\u03bcs';
```

## 3 Source file structure

### 3.1 ES modules

ES modules are files that use the `import` and `export` keywords.

#### 3.1.1 Imports

Import statements must not be line wrapped and are therefore an exception to the 80-column limit.

##### 3.1.1.1 Import paths

ES module files must use the `import` statement to import other ES module files.

```js
import './sideeffects.js';
import * as parent from '../parent.js';
import {name} from './sibling.js';
```

###### 3.1.1.1.1 File extensions in import paths

The `.js` file extension is not optional in import paths and must always be included.

```js
// Bad
import '../directory/file';

// Good
import '../directory/file.js';
```

##### 3.1.1.2 Importing the same file multiple times

Do not import the same file multiple times. This can make it hard to determine the aggregate imports of a file.

```js
// Bad - same path imported multiple times
import {short} from './long/path/to/a/file.js';
import {aLongNameThatBreaksAlignment} from './long/path/to/a/file.js';
```

##### 3.1.1.3 Naming imports

###### 3.1.1.3.1 Naming module imports

Module import names (`import * as name`) are `lowerCamelCase` names that are derived from the imported file name.

```js
import * as fileOne from '../file-one.js';
import * as fileTwo from '../file_two.js';
import * as fileThree from '../filethree.js';
```

###### 3.1.1.3.2 Naming default imports

Default import names are derived from the imported file name and follow the naming rules.

```js
import MyClass from '../my-class.js';
import myFunction from '../my_function.js';
import SOME_CONSTANT from '../someconstant.js';
```

###### 3.1.1.3.3 Naming named imports

In general, symbols imported via the named import (`import {name}`) should keep the same name. Avoid aliasing imports (`import {SomeThing as SomeOtherThing}`). Prefer fixing name collisions by using a module import (`import *`) or renaming the exports themselves.

```js
import * as bigAnimals from './biganimals.js';
import * as domesticatedAnimals from './domesticatedanimals.js';

new bigAnimals.Cat();
new domesticatedAnimals.Cat();
```

#### 3.1.2 Exports

Symbols are only exported if they are meant to be used outside the module. Non-exported module-local symbols are not declared private.

##### 3.1.2.1 Named vs default exports

Use named exports in all code. You can apply the `export` keyword to a declaration, or use the `export {name};` syntax.

Do not use default exports. Importing modules must give a name to these values, which can lead to inconsistencies in naming across modules.

```js
// Bad - do not use default exports:
export default class Foo { ... }

// Good - use named exports:
export class Foo { ... }

// Alternate style named exports:
class Foo { ... }
export {Foo};
```

##### 3.1.2.2 Mutability of exports

Exported variables must not be mutated outside of module initialization.

```js
// Bad: both foo and mutateFoo are exported and mutated.
export let foo = 0;
export function mutateFoo() {
  ++foo;
}

// Good: Use getter functions instead
let foo = 0;
export function getFoo() {
  return foo;
}
export function mutateFoo() {
  foo = mutateFooFunc(foo);
}
```

##### 3.1.2.3 export from

`export from` statements must not be line wrapped and are therefore an exception to the 80-column limit.

```js
export {specificName} from './other.js';
export * from './another.js';
```

#### 3.1.3 Circular Dependencies in ES modules

Do not create cycles between ES modules, even though the ECMAScript specification allows this.

```js
// a.js - Bad
import './b.js';

// b.js - Bad
import './a.js';
export {x} from './c.js';

// c.js - Bad
import './b.js';
export let x;
```

## 4 Formatting

**Terminology Note**: *block-like construct* refers to the body of a class, function, method, or brace-delimited block of code.

### 4.1 Braces

#### 4.1.1 Braces are used for all control structures

Braces are required for all control structures (i.e. `if`, `else`, `for`, `do`, `while`, as well as any others), even if the body contains only a single statement.

```js
// Bad
if (someVeryLongCondition())
  doSomething();

for (let i = 0; i < foo.length; i++) bar(foo[i]);
```

**Exception**: A simple if statement that can fit entirely on a single line with no wrapping (and that doesn't have an else) may be kept on a single line with no braces when it improves readability.

```js
if (shortCondition()) foo();
```

#### 4.1.2 Nonempty blocks: K&R style

Braces follow the Kernighan and Ritchie style ("Egyptian brackets") for *nonempty* blocks and block-like constructs:

- No line break before the opening brace.
- Line break after the opening brace.
- Line break before the closing brace.
- Line break after the closing brace *if* that brace terminates a statement or the body of a function or class statement, or a class method.

```js
class InnerClass {
  constructor() {}

  method(foo) {
    if (condition(foo)) {
      try {
        // Note: this might fail.
        something();
      } catch (err) {
        recover();
      }
    }
  }
}
```

#### 4.1.3 Empty blocks: may be concise

An empty block or block-like construct *may* be closed immediately after it is opened, with no characters, space, or line break in between (i.e. `{}`), **unless** it is a part of a *multi-block statement*.

```js
// Good
function doNothing() {}

// Bad
if (condition) {
  // …
} else if (otherCondition) {} else {
  // …
}
```

### 4.2 Block indentation: +2 spaces

Each time a new block or block-like construct is opened, the indent increases by two spaces. When the block ends, the indent returns to the previous indent level.

#### 4.2.1 Array literals: optionally "block-like"

Any array literal may optionally be formatted as if it were a "block-like construct."

```js
const a = [
  0,
  1,
  2,
];

const b = [0, 1, 2];

someMethod(foo, [
  0, 1, 2,
], bar);
```

#### 4.2.2 Object literals: optionally "block-like"

Any object literal may optionally be formatted as if it were a "block-like construct."

```js
const a = {
  a: 0,
  b: 1,
};

const b = {a: 0, b: 1};

someMethod(foo, {
  a: 0, b: 1,
}, bar);
```

#### 4.2.3 Class literals

Class literals (whether declarations or expressions) are indented as blocks. Do not add semicolons after methods, or after the closing brace of a class *declaration*.

```js
class Foo {
  constructor(x) {
    this.x = x;
  }
}

class Bar extends Foo {
  constructor() {
    super(42);
  }
}

exports.Baz = class extends Bar {
  method() {
    return this.x;
  }
};
```

#### 4.2.4 Function expressions

When declaring an anonymous function in the list of arguments for a function call, the body of the function is indented two spaces more than the preceding indentation depth.

```js
prefix.something.reallyLongFunctionName('whatever', (a1, a2) => {
  // Indent the function body +2 relative to indentation depth
  // of the 'prefix' statement one line above.
  if (a1.equals(a2)) {
    someOtherLongFunctionName(a1);
  } else {
    andNowForSomethingCompletelyDifferent(a2.parrot);
  }
});
```

#### 4.2.5 Switch statements

As with any other block, the contents of a switch block are indented +2.

After a switch label, a newline appears, and the indentation level is increased +2. The following switch label returns to the previous indentation level.

```js
switch (animal) {
  case Animal.BANDERSNATCH:
    handleBandersnatch();
    break;

  case Animal.JABBERWOCK:
    handleJabberwock();
    break;

  default:
    throw new Error('Unknown animal');
}
```

### 4.3 Statements

#### 4.3.1 One statement per line

Each statement is followed by a line-break.

#### 4.3.2 Semicolons are required

Every statement must be terminated with a semicolon. Relying on automatic semicolon insertion is forbidden.

### 4.4 Column limit: 80

JavaScript code has a column limit of 80 characters. Except as noted below, any line that would exceed this limit must be line-wrapped.

**Exceptions:**
1. ES module `import` and `export from` statements
2. Lines where obeying the column limit is not possible or would hinder discoverability:
    - A long URL which should be clickable in source
    - A shell command intended to be copied-and-pasted
    - A long string literal which may need to be copied or searched for wholly

### 4.5 Line-wrapping

**Terminology Note**: *Line wrapping* is breaking a chunk of code into multiple lines to obey column limit.

#### 4.5.1 Where to break

The prime directive of line-wrapping is: prefer to break at a **higher syntactic level**.

```js
// Preferred
currentEstimate =
    calc(currentEstimate + x * currentEstimate) /
        2.0;

// Discouraged
currentEstimate = calc(currentEstimate + x *
    currentEstimate) / 2.0;
```

Operators are wrapped as follows:
1. When a line is broken at an operator the break comes after the symbol
2. A method or constructor name stays attached to the open parenthesis `(` that follows it
3. A comma `,` stays attached to the token that precedes it
4. A line break is never added between a return and the return value

#### 4.5.2 Indent continuation lines at least +4 spaces

When line-wrapping, each line after the first (each *continuation line*) is indented at least +4 from the original line, unless it falls under the rules of block indentation.

### 4.6 Whitespace

#### 4.6.1 Vertical whitespace

A single blank line appears:
1. Between consecutive methods in a class or object literal
2. Within method bodies, sparingly to create *logical groupings* of statements
3. *Optionally* before the first or after the last method in a class or object literal
4. As required by other sections of this document

#### 4.6.2 Horizontal whitespace

Use of horizontal whitespace depends on location. Leading whitespace (i.e., indentation) is addressed elsewhere. Trailing whitespace is forbidden.

A single internal ASCII space appears in the following places **only**:
1. Separating any reserved word (such as `if`, `for`, or `catch`) except for `function` and `super`, from an open parenthesis `(` that follows it on that line
2. Separating any reserved word (such as `else` or `catch`) from a closing curly brace `}` that precedes it on that line
3. Before any open curly brace `{`, with two exceptions:
    - Before an object literal that is the first argument of a function
    - In a template expansion (e.g. valid: `` `ab${1 + 2}cd` ``)
4. On both sides of any binary or ternary operator
5. After a comma `,` or semicolon `;`
6. After the colon `:` in an object literal
7. On both sides of the double slash `//` that begins an end-of-line comment

#### 4.6.3 Horizontal alignment: discouraged

**Horizontal alignment** is the practice of adding a variable number of additional spaces in your code with the goal of making certain tokens appear directly below certain other tokens on previous lines.

This practice is permitted, but it is **generally discouraged**.

```js
{
  tiny: 42, // this is great
  longer: 435, // this too
}

{
  tiny:   42,  // permitted, but future edits
  longer: 435, // may leave it unaligned
}
```

#### 4.6.4 Function arguments

Prefer to put all function arguments on the same line as the function name. If doing so would exceed the 80-column limit, the arguments must be line-wrapped in a readable way.

```js
// Arguments start on a new line, indented four spaces
doSomething(
    descriptiveArgumentOne, descriptiveArgumentTwo, descriptiveArgumentThree) {
  // …
}

// Four-space, one argument per line
doSomething(
    veryDescriptiveArgumentNumberOne,
    veryDescriptiveArgumentTwo,
    tableModelEventHandlerProxy,
    artichokeDescriptorAdapterIterator) {
  // …
}
```

### 4.7 Grouping parentheses: recommended

Optional grouping parentheses are omitted only when the author and reviewer agree that there is no reasonable chance that the code will be misinterpreted without them.

Do not use unnecessary parentheses around the entire expression following `delete`, `typeof`, `void`, `return`, `throw`, `case`, `in`, `of`, or `yield`.

### 4.8 Comments

#### 4.8.1 Block comment style

Block comments are indented at the same level as the surrounding code. They may be in `/* … */` or `//`-style.

```js
/*
 * This is
 * okay.
 */

// And so
// is this.

/* This is fine, too. */
```

#### 4.8.2 Parameter Name Comments

"Parameter name" comments should be used whenever the value and method name do not sufficiently convey the meaning.

```js
someFunction(obviousParam, /* shouldRender= */ true, /* name= */ 'hello');
```

## 5 Language features

### 5.1 Local variable declarations

#### 5.1.1 Use `const` and `let`

Declare all local variables with either `const` or `let`. Use `const` by default, unless a variable needs to be reassigned. The `var` keyword must not be used.

#### 5.1.2 One variable per declaration

Every local variable declaration declares only one variable: declarations such as `let a = 1, b = 2;` are not used.

#### 5.1.3 Declared when needed, initialized as soon as possible

Local variables are **not** habitually declared at the start of their containing block. Instead, local variables are declared close to the point they are first used (within reason), to minimize their scope.

### 5.2 Array literals

#### 5.2.1 Use trailing commas

Include a trailing comma whenever there is a line break between the final element and the closing bracket.

```js
const values = [
  'first value',
  'second value',
];
```

#### 5.2.2 Do not use the variadic `Array` constructor

The constructor is error-prone if arguments are added or removed. Use a literal instead.

```js
// Bad
const a1 = new Array(x1, x2, x3);
const a2 = new Array(x1, x2);
const a3 = new Array(x1);
const a4 = new Array();

// Good
const a1 = [x1, x2, x3];
const a2 = [x1, x2];
const a3 = [x1];
const a4 = [];
```

#### 5.2.3 Non-numeric properties

Do not define or use non-numeric properties on an array (other than `length`). Use a `Map` (or `Object`) instead.

#### 5.2.4 Destructuring

Array literals may be used on the left-hand side of an assignment to perform destructuring.

```js
const [a, b, c, ...rest] = generateResults();
let [, b,, d] = someArray;

// Function parameters with default values
function optionalDestructuring([a = 4, b = 2] = []) { … }
```

#### 5.2.5 Spread operator

Array literals may include the spread operator (`...`) to flatten elements out of one or more other iterables.

```js
[...foo]   // preferred over Array.prototype.slice.call(foo)
[...foo, ...bar]   // preferred over foo.concat(bar)
```

### 5.3 Object literals

#### 5.3.1 Use trailing commas

Include a trailing comma whenever there is a line break between the final property and the closing brace.

#### 5.3.2 Do not use the `Object` constructor

Use an object literal (`{}` or `{a: 0, b: 1, c: 2}`) instead.

#### 5.3.3 Do not mix quoted and unquoted keys

Object literals may represent either *structs* (with unquoted keys and/or symbols) or *dicts* (with quoted and/or computed keys). Do not mix these key types in a single object literal.

```js
// Bad
{
  width: 42, // struct-style unquoted key
  'maxWidth': 43, // dict-style quoted key
}
```

#### 5.3.4 Computed property names

Computed property names (e.g., `{['key' + foo()]: 42}`) are allowed, and are considered dict-style (quoted) keys.

#### 5.3.5 Method shorthand

Methods can be defined on object literals using the method shorthand (`{method() {… }}`) in place of a colon immediately followed by a `function` or arrow function literal.

```js
return {
  stuff: 'candy',
  method() {
    return this.stuff;  // Returns 'candy'
  },
};
```

#### 5.3.6 Shorthand properties

Shorthand properties are allowed on object literals.

```js
const foo = 1;
const bar = 2;
const obj = {
  foo,
  bar,
  method() { return this.foo + this.bar; },
};
```

#### 5.3.7 Destructuring

Object destructuring patterns may be used on the left-hand side of an assignment to perform destructuring and unpack multiple values from a single object.

```js
function destructured(ordinary, {num, str = 'some default'} = {}) {}
```

#### 5.3.8 Enums

Enumerations are defined by adding an enum annotation to an object literal. All enum values must be either a string literal or a number.

```js
/**
 * Supported temperature scales.
 * @enum {string}
 */
const TemperatureScale = {
  CELSIUS: 'celsius',
  FAHRENHEIT: 'fahrenheit',
};
```

### 5.4 Classes

#### 5.4.1 Constructors

Constructors are optional. Subclass constructors must call `super()` before setting any fields or otherwise accessing `this`.

#### 5.4.2 Fields

Define all of a concrete object's fields (i.e. all properties other than methods) in the constructor. Fields are never defined on a concrete class's `prototype`.

```js
class Foo {
  constructor() {
    this.bar_ = computeBar();
    this.baz = computeBaz();
  }
}
```

#### 5.4.3 Computed properties

Computed properties may only be used in classes when the property is a symbol. Dict-style properties (that is, quoted or computed non-symbol keys) are not allowed.

#### 5.4.4 Static methods

Where it does not interfere with readability, prefer module-local functions over private static methods.

#### 5.4.5 Do not manipulate `prototype`s directly

The `class` keyword allows clearer and more readable class definitions than defining `prototype` properties.

#### 5.4.6 Getters and Setters

Do not use JavaScript getter and setter properties. They are potentially surprising and difficult to reason about. Provide ordinary methods instead.

#### 5.4.7 Overriding toString

The `toString` method may be overridden, but must always succeed and never have visible side effects.

#### 5.4.8 Interfaces

Interfaces may be declared with `@interface` or `@record`. All methods on an interface must be non-static and method bodies must be empty blocks.

#### 5.4.9 Abstract Classes

Use abstract classes when appropriate. Abstract classes and methods must be annotated appropriately.

### 5.5 Functions

#### 5.5.1 Top-level functions

Top-level functions may be defined directly on the `exports` object, or else declared locally and optionally exported.

```js
// Option 1
exports.processString = (str) => {
  // Process the string.
};

// Option 2
const processString = (str) => {
  // Process the string.
};
exports = {processString};
```

#### 5.5.2 Nested functions and closures

Functions may contain nested function definitions. If it is useful to give the function a name, it should be assigned to a local `const`.

#### 5.5.3 Arrow functions

Arrow functions provide a concise function syntax and simplify scoping `this` for nested functions. Prefer arrow functions over the `function` keyword for nested functions.

```js
const moduleLocalFunc = (numParam, strParam) => numParam + Number(strParam);

getValue((result) => void alert(`Got ${result}`));

class CallbackExample {
  constructor() {
    this.cachedValue_ = 0;
    getNullableValue((result) => {
      this.cachedValue_ = result == null ? 0 : result;
    });
  }
}
```

#### 5.5.4 Generators

Generators enable a number of useful abstractions and may be used as needed.

```js
function* gen1() {
  yield 42;
}

const gen2 = function*() {
  yield* gen1();
}

class SomeClass {
  * gen() {
    yield 42;
  }
}
```

#### 5.5.5 Parameter and return types

##### 5.5.5.1 Default parameters

Optional parameters are permitted using the equals operator in the parameter list.

```js
function maybeDoSomething(required, optional = '', node = undefined) {}
```

##### 5.5.5.2 Rest parameters

Use a *rest* parameter instead of accessing `arguments`. The rest parameter must be the last parameter in the list.

```js
function variadic(array, ...numbers) {}
```

#### 5.5.6 Spread operator

Function calls may use the spread operator (`...`).

```js
function myFunction(...elements) {}
myFunction(...array, ...iterable, ...generator());
```

### 5.6 String literals

#### 5.6.1 Use single quotes

Ordinary string literals are delimited with single quotes (`'`), rather than double quotes (`"`).

#### 5.6.2 Template literals

Use template literals (delimited with `` ` ``) over complex string concatenation, particularly if multiple string literals are involved.

```js
function arithmetic(a, b) {
  return `Here is a table of arithmetic operations:
${a} + ${b} = ${a + b}
${a} - ${b} = ${a - b}
${a} * ${b} = ${a * b}
${a} / ${b} = ${a / b}`;
}
```

#### 5.6.3 No line continuations

Do not use *line continuations* (ending a line inside a string literal with a backslash).

```js
// Bad
const longString = 'This is a very long string that far exceeds the 80 \
    column limit. It unfortunately contains long stretches of spaces due \
    to how the continued lines are indented.';

// Good
const longString = 'This is a very long string that far exceeds the 80 ' +
    'column limit. It does not contain long stretches of spaces since ' +
    'the concatenated strings are cleaner.';
```

### 5.7 Number literals

Numbers may be specified in decimal, hex, octal, or binary. Use exactly `0x`, `0o`, and `0b` prefixes, with lowercase letters, for hex, octal, and binary, respectively.

### 5.8 Control structures

#### 5.8.1 For loops

With ES6, the language now has three different kinds of `for` loops. All may be used, though `for`-`of` loops should be preferred when possible.

#### 5.8.2 Exceptions

Exceptions are an important part of the language and should be used whenever exceptional cases occur. Always throw `Error`s or subclasses of `Error`: never throw string literals or other objects. Always use `new` when constructing an `Error`.

##### 5.8.2.1 Empty catch blocks

It is very rarely correct to do nothing in response to a caught exception. When it truly is appropriate to take no action whatsoever in a catch block, the reason this is justified is explained in a comment.

```js
try {
  return handleNumericResponse(response);
} catch (ok) {
  // it's not numeric; that's fine, just continue
}
return handleTextResponse(response);
```

#### 5.8.3 Switch statements

##### 5.8.3.1 Fall-through: commented

Within a switch block, each statement group either terminates abruptly (with a `break`, `return` or `throw`n exception), or is marked with a comment to indicate that execution will or might continue into the next statement group.

```js
switch (input) {
  case 1:
  case 2:
    prepareOneOrTwo();
  // fall through
  case 3:
    handleOneTwoOrThree();
    break;
  default:
    handleLargeNumber(input);
}
```

##### 5.8.3.2 The `default` case is present

Each switch statement includes a `default` statement group, even if it contains no code.

### 5.9 this

Only use `this` in class constructors and methods, in arrow functions defined within class constructors and methods, or in functions that have an explicit `@this` declared.

### 5.10 Equality Checks

Use identity operators (`===`/`!==`) except in the cases documented below.

#### 5.10.1 Exceptions Where Coercion is Desirable

Catching both `null` and `undefined` values:

```js
if (someObjectOrPrimitive == null) {
  // Checking for null catches both null and undefined
}
```

### 5.11 Disallowed features

#### 5.11.1 with

Do not use the `with` keyword. It makes your code harder to understand and has been banned in strict mode since ES5.

#### 5.11.2 Dynamic code evaluation

Do not use `eval` or the `Function(...string)` constructor. These features are potentially dangerous and simply do not work in CSP environments.

#### 5.11.3 Automatic semicolon insertion

Always terminate statements with semicolons.

#### 5.11.4 Non-standard features

Do not use non-standard features. Use only features defined in the current ECMA-262 or WHATWG standards.

#### 5.11.5 Wrapper objects for primitive types

Never use `new` on the primitive object wrappers (`Boolean`, `Number`, `String`, `Symbol`).

```js
// Bad
const x = new Boolean(false);
if (x) alert(typeof x);  // alerts 'object' - WAT?

// Good
const x = Boolean(0);
if (!x) alert(typeof x);  // alerts 'boolean', as expected
```

#### 5.11.6 Modifying builtin objects

Never modify builtin types, either by adding methods to their constructors or to their prototypes.

#### 5.11.7 Omitting `()` when invoking a constructor

Never invoke a constructor in a `new` statement without using parentheses `()`.

```js
// Bad
new Foo;

// Good
new Foo();
```

## 6 Naming

### 6.1 Rules common to all identifiers

Identifiers use only ASCII letters and digits, and, in a small number of cases noted below, underscores and very rarely dollar signs.

Give as descriptive a name as possible, within reason. Do not worry about saving horizontal space as it is far more important to make your code immediately understandable by a new reader.

```js
// Good
errorCount          // No abbreviation.
dnsConnectionIndex  // Most people know what "DNS" stands for.
referrerUrl         // Ditto for "URL".
customerId          // "Id" is both ubiquitous and unlikely to be misunderstood.

// Bad
n                   // Meaningless.
nErr                // Ambiguous abbreviation.
nCompConns          // Ambiguous abbreviation.
wgcConnections      // Only your group knows what this stands for.
pcReader            // Lots of things can be abbreviated "pc".
cstmrId             // Deletes internal letters.
kSecondsPerDay      // Do not use Hungarian notation.
```

### 6.2 Rules by identifier type

#### 6.2.1 Package names

Package names are all `lowerCamelCase`.

#### 6.2.2 Class names

Class, interface, record, and typedef names are written in `UpperCamelCase`. Type names are typically nouns or noun phrases.

#### 6.2.3 Method names

Method names are written in `lowerCamelCase`. Method names are typically verbs or verb phrases.

#### 6.2.4 Enum names

Enum names are written in `UpperCamelCase`, similar to classes, and should generally be singular nouns. Individual items within the enum are named in `CONSTANT_CASE`.

#### 6.2.5 Constant names

Constant names use `CONSTANT_CASE`: all uppercase letters, with words separated by underscores.

##### 6.2.5.1 Definition of "constant"

Every constant is a `const` declaration, but not all `const`s are constants. Before choosing constant case, consider whether the field really feels like a *deeply immutable* constant.

```js
// Constants
const NUMBER = 5;
const NAMES = Object.freeze(['Ed', 'Ann']);

// Not constants
let letVariable = 'non-const';
const mutableCollection = new Set();
const logger = log.getLogger('loggers.are.not.immutable');
```

#### 6.2.6 Non-constant field names

Non-constant field names (static or otherwise) are written in `lowerCamelCase`, with an optional trailing underscore for private fields.

#### 6.2.7 Parameter names

Parameter names are written in `lowerCamelCase`. One-character parameter names should not be used in public methods.

#### 6.2.8 Local variable names

Local variable names are written in `lowerCamelCase`. Constants in function scopes are still named in `lowerCamelCase`.

### 6.3 Camel case: defined

Sometimes there is more than one reasonable way to convert an English phrase into camel case, such as when acronyms or unusual constructs like "IPv6" or "iOS" are present.

Beginning with the prose form of the name:
1. Convert the phrase to plain ASCII and remove any apostrophes
2. Divide this result into words, splitting on spaces and any remaining punctuation
3. Now lowercase everything (including acronyms), then uppercase only the first character of:
    - each word, to yield `UpperCamelCase`, or
    - each word except the first, to yield `lowerCamelCase`
4. Finally, join all the words into a single identifier

Examples:

| Prose form | Correct | Incorrect |
|------------|---------|-----------|
| "XML HTTP request" | `xmlHttpRequest` | `XMLHTTPRequest` |
| "new customer ID" | `newCustomerId` | `newCustomerID` |
| "inner stopwatch" | `innerStopwatch` | `innerStopWatch` |
| "supports IPv6 on iOS?" | `supportsIpv6OnIos` | `supportsIPv6OnIOS` |

## 7 Policies

### 7.1 Issues unspecified by Style Guide: Be Consistent!

For any style question that isn't settled definitively by this specification, prefer to do what the other code in the same file is already doing.

### 7.2 Code not in proper style

When updating the style of existing code:
1. It is not required to change all existing code to meet current style guidelines
2. Be careful not to allow opportunistic style fixes to muddle the focus of a change

Brand new files use proper style, regardless of the style choices of other files in the same package.

### 7.3 Local style rules

Teams and projects may adopt additional style rules beyond those in this document, but must accept that cleanup changes may not abide by these additional rules.

### 7.4 Generated code: mostly exempt

Source code generated by the build process is not required to be in proper style. However, any generated identifiers that will be referenced from hand-written source code must follow the naming requirements.