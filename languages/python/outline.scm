; Function definitions
(function_definition
  name: (identifier) @name
  (#set! "kind" "function")) @item

; Class definitions
(class_definition
  name: (identifier) @name
  (#set! "kind" "class")) @item

; Method definitions within classes
(class_definition
  body: (block
    (function_definition
      name: (identifier) @name
      (#set! "kind" "method")))) @item

; Variable assignments (simple)
(assignment
  left: (identifier) @name
  (#set! "kind" "variable")) @item

; Import statements
(import_statement
  (dotted_as_names
    (dotted_as_name
      alias: (identifier) @name
      (#set! "kind" "import")))) @item

(import_statement
  (aliased_import
    alias: (identifier) @name
    (#set! "kind" "import"))) @item

; From import statements
(import_from_statement
  (dotted_as_names
    (dotted_as_name
      alias: (identifier) @name
      (#set! "kind" "import")))) @item

(import_from_statement
  (aliased_import
    alias: (identifier) @name
    (#set! "kind" "import"))) @item

; Global variable declarations
(global_statement
  (identifier) @name
  (#set! "kind" "global")) @item

; Nonlocal variable declarations
(nonlocal_statement
  (identifier) @name
  (#set! "kind" "nonlocal")) @item