# Rust EBNF (Practical Subset)

This is a mid-sized, practical Rust grammar. It is larger than our toy subset
but still compact enough to implement. It aims to cover common, everyday Rust
syntax without the full complexity of the official reference grammar.

The grammar is EBNF-like and intended as a guide for implementation, not an
authoritative spec.

## Lexical

```ebnf
Ident       = Letter, { Letter | Digit | "_" } ;
Int         = Digit, { Digit } ;
Float       = Digit, { Digit }, ".", Digit, { Digit } ;
Char        = "'", ( ? any char except ' or \\ ? | Escape ), "'" ;
String      = "\"", { ? any char except \" or \\ ? | Escape }, "\"" ;
Escape      = "\\", ( "n" | "r" | "t" | "0" | "\\" | "\"" | "'" ) ;

Letter      = "A".."Z" | "a".."z" | "_" ;
Digit       = "0".."9" ;
```

## Program

```ebnf
Program     = { Item } ;
Item        = Function | Struct | Enum | Impl | Use | Mod | Const | Static ;

Use         = "use", Path, ";" ;
Mod         = "mod", Ident, ";" ;
Const       = "const", Ident, ":", Type, "=", Expr, ";" ;
Static      = "static", [ "mut" ], Ident, ":", Type, "=", Expr, ";" ;
```

## Paths

```ebnf
Path        = PathSeg, { "::", PathSeg } ;
PathSeg     = Ident | "self" | "super" | "crate" ;
```

## Types

```ebnf
Type        = TypePath
            | "(" [ TypeList ] ")"
            | "[", Type, ";", Expr, "]"
            | "&", [ "mut" ], Type
            | "*", [ "const" | "mut" ], Type
            | "fn", "(", [ TypeList ], ")", "->", Type ;

TypePath    = Path ;
TypeList    = Type, { ",", Type } ;
```

## Functions

```ebnf
Function    = "fn", Ident, "(", [ ParamList ], ")", [ "->", Type ], Block ;
ParamList   = Param, { ",", Param } ;
Param       = Pattern, ":", Type ;
```

## Structs / Enums / Impl

```ebnf
Struct      = "struct", Ident, StructBody, [ ";" ] ;
StructBody  = "{", [ FieldList ], "}" | "(", [ TypeList ], ")" ;
FieldList   = Field, { ",", Field }, [ "," ] ;
Field       = Ident, ":", Type ;

Enum        = "enum", Ident, "{", [ VariantList ], "}" ;
VariantList = Variant, { ",", Variant }, [ "," ] ;
Variant     = Ident, [ StructBody ] ;

Impl        = "impl", Type, "{", { ImplItem }, "}" ;
ImplItem    = Function | Const | TypeAlias ;
TypeAlias   = "type", Ident, "=", Type, ";" ;
```

## Patterns

```ebnf
Pattern     = Ident
            | "_"
            | "ref", Ident
            | "mut", Ident
            | "&", [ "mut" ], Pattern
            | "(", [ PatternList ], ")"
            | Path
            | Literal ;

PatternList = Pattern, { ",", Pattern } ;
```

## Statements and Blocks

```ebnf
Block       = "{", { Stmt }, [ Expr ], "}" ;
Stmt        = LetStmt | ExprStmt | Item ;

LetStmt     = "let", Pattern, [ ":", Type ], [ "=", Expr ], ";" ;
ExprStmt    = Expr, ";" ;
```

## Expressions (precedence)

```ebnf
Expr        = IfExpr | LoopExpr | MatchExpr | OrExpr ;

IfExpr      = "if", Expr, Block, [ "else", ( IfExpr | Block ) ] ;
LoopExpr    = "loop", Block
            | "while", Expr, Block
            | "for", Pattern, "in", Expr, Block ;

MatchExpr   = "match", Expr, "{", { MatchArm }, "}" ;
MatchArm    = Pattern, [ "if", Expr ], "=>", ( Expr | Block ), [ "," ] ;

OrExpr      = AndExpr, { "||", AndExpr } ;
AndExpr     = EqExpr, { "&&", EqExpr } ;
EqExpr      = CmpExpr, { ( "==" | "!=" ), CmpExpr } ;
CmpExpr     = AddExpr, { ( "<" | "<=" | ">" | ">=" ), AddExpr } ;
AddExpr     = MulExpr, { ( "+" | "-" ), MulExpr } ;
MulExpr     = UnaryExpr, { ( "*" | "/" | "%" ), UnaryExpr } ;

UnaryExpr   = ( "!" | "-" | "*" | "&" | "&mut" ), UnaryExpr
            | PostfixExpr ;

PostfixExpr = PrimaryExpr, { PostfixOp } ;
PostfixOp   = Call | Index | Field | MethodCall ;
Call        = "(", [ ExprList ], ")" ;
Index       = "[", Expr, "]" ;
Field       = ".", Ident ;
MethodCall  = ".", Ident, "(", [ ExprList ], ")" ;

PrimaryExpr = Literal
            | Path
            | "(", [ ExprList ], ")"
            | Block ;

ExprList    = Expr, { ",", Expr } ;
```

## Literals

```ebnf
Literal     = Int | Float | Char | String | "true" | "false" ;
```

## Notes
- This grammar omits macros, lifetimes, generics, traits, attributes, and many
  advanced features.
- It is intended for building a compact compiler while still supporting most
  common Rust syntax found in small programs.
