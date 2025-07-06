; Function declarations
(function_declaration
  name: (identifier) @name
  (#set! "kind" "function")) @item

; Function expressions
(function_expression
  name: (identifier) @name
  (#set! "kind" "function")) @item

; Arrow functions assigned to variables
(variable_declarator
  name: (identifier) @name
  value: (arrow_function)
  (#set! "kind" "function")) @item

; Variable declarations
(variable_declaration
  (variable_declarator
    name: (identifier) @name
    (#set! "kind" "variable"))) @item

; Class declarations
(class_declaration
  name: (identifier) @name
  (#set! "kind" "class")) @item

; Method definitions
(method_definition
  name: (property_identifier) @name
  (#set! "kind" "method")) @item

; Object property shorthand
(pair
  key: (property_identifier) @name
  value: (function_expression)
  (#set! "kind" "method")) @item

; Object method shorthand
(pair
  key: (property_identifier) @name
  value: (arrow_function)
  (#set! "kind" "method")) @item