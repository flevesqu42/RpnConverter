**Description:**

Take already splited standard notation as mutable VeqDeque, and convert it into a Vec with parenthesis removal.

For example:

`( ( A | B ) + C ) + ( D ^ E ) + ( F | G | H )`

Will give:

`A B | C + D E ^ + F G | H | +`


**Usage and examples:**

Instantiate modular converter with:

```RpnConverter::new(open_parenthesis, closing_parenthesis, unary_operands, binary_operands)```

Remove parenthesis with:

```converter.remove_parenthesis(standard_notation)```

Given:

```
let standard_notation = "( ( A + B ) | ( D | ( ( K + O ) ^ A ) ) ) ^ B".split_ascii_whitespace().collect();
```

Usage will be:
```
let converter = RpnConverter::new(["("].to_vec(), [")"].to_vec(), ["!"].to_vec(), ["+", "^", "|"].to_vec());
let result = converter.remove_parenthesis(standard_notation)?;
```

Previous code will attempt result as:

```
let attempted_result : Vec<&str> = "A B + D K O + A ^ | | B ^".split_ascii_whitespace().collect();
```

See more usage examples in tests.rs.
