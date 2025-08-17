# Typing "." in if statements does not print the value
# instead it returns the value for the if statement

[[ . == "4" ]] # The start of an if statement
  # Do stuff
  ++++
[[ . == "3" ]] # Else if
  +++
[[]] # Else
  ++++
[] # End if

# Yes you can use || (OR) and && (AND)
