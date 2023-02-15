


## Example Usages

### Value
- value create "key" "value"              | true
- value create "key" "new_value"          | false
- value read "key"                        | "value"
- value update "key" "new_value"          | true
- value read "key"                        | "new_value"
- value delete "key"                      | true
- value delete "key"                      | false

### List
- list push "list1" "value"               | true
- list read "list1"                       | ["value"]   
- list filter "list1" "%alu%"             | ["value"]
- list has "list1" "value"                | true
- list index "list1" "value"              | 0
- list remove "list1" "value"             | true
- list read "list1"                       | []
- ...



    
