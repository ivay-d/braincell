{
  fn braincell(number) # Define args
  # For multiple args you would do
  # braincell(number, word)
  [[ number == 4 ]]
    ++++
  [[]]
    ++
  []
}

@braincell(4) # Call function with the args
# If you did @braincell() it would throw an error
